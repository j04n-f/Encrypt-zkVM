// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::slice;

use winterfell::{
    crypto::{Digest, Hasher},
    math::{fields::f128::BaseElement, FieldElement},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

pub const STATE_WIDTH: usize = 4;
pub const RATE_WIDTH: usize = 2;

const DIGEST_SIZE: usize = 2;

pub const NUM_ROUNDS: usize = 14;

pub const CYCLE_LENGTH: usize = 16;

pub struct Rescue128 {
    state: [BaseElement; STATE_WIDTH],
    idx: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Hash([BaseElement; DIGEST_SIZE]);

impl Default for Rescue128 {
    fn default() -> Self {
        Self::new()
    }
}

impl Rescue128 {
    pub fn new() -> Self {
        Rescue128 {
            state: [BaseElement::ZERO; STATE_WIDTH],
            idx: 0,
        }
    }

    pub fn state(&self) -> [BaseElement; STATE_WIDTH] {
        self.state
    }

    pub fn update(&mut self, data: &[BaseElement]) {
        for &element in data {
            self.state[self.idx] += element;
            self.idx += 1;
            if self.idx % RATE_WIDTH == 0 {
                apply_permutation(&mut self.state);
                self.idx = 0;
            }
        }
    }

    pub fn finalize(mut self) -> Hash {
        if self.idx > 0 {
            apply_permutation(&mut self.state);
        }
        Hash([self.state[0], self.state[1]])
    }

    pub fn digest(data: &[BaseElement]) -> Hash {
        let mut state = [BaseElement::ZERO; STATE_WIDTH];

        let mut i = 0;
        for &element in data.iter() {
            state[i] += element;
            i += 1;
            if i % RATE_WIDTH == 0 {
                apply_permutation(&mut state);
                i = 0;
            }
        }

        if i > 0 {
            apply_permutation(&mut state);
        }

        Hash([state[0], state[1]])
    }
}

impl Hasher for Rescue128 {
    type Digest = Hash;

    const COLLISION_RESISTANCE: u32 = 64;

    fn hash(_bytes: &[u8]) -> Self::Digest {
        unimplemented!("not implemented")
    }

    fn merge(values: &[Self::Digest; 2]) -> Self::Digest {
        Self::digest(Hash::hashes_as_elements(values))
    }

    fn merge_with_int(_seed: Self::Digest, _value: u64) -> Self::Digest {
        unimplemented!("not implemented")
    }
}

impl Hash {
    pub fn new(v1: BaseElement, v2: BaseElement) -> Self {
        Hash([v1, v2])
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0; 32];
        bytes[..16].copy_from_slice(&self.0[0].to_bytes());
        bytes[16..].copy_from_slice(&self.0[1].to_bytes());
        bytes
    }

    pub fn to_elements(&self) -> [BaseElement; DIGEST_SIZE] {
        self.0
    }

    pub fn hashes_as_elements(hashes: &[Hash]) -> &[BaseElement] {
        let p = hashes.as_ptr();
        let len = hashes.len() * DIGEST_SIZE;
        unsafe { slice::from_raw_parts(p as *const BaseElement, len) }
    }
}

impl Digest for Hash {
    fn as_bytes(&self) -> [u8; 32] {
        let bytes = BaseElement::elements_as_bytes(&self.0);
        let mut result = [0; 32];
        result[..bytes.len()].copy_from_slice(bytes);
        result
    }
}

impl Serializable for Hash {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.0[0]);
        target.write(self.0[1]);
    }
}

impl Deserializable for Hash {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let v1 = BaseElement::read_from(source)?;
        let v2 = BaseElement::read_from(source)?;
        Ok(Self([v1, v2]))
    }
}

pub fn apply_permutation(state: &mut [BaseElement; STATE_WIDTH]) {
    for i in 0..NUM_ROUNDS {
        // apply_round(state, i);
    }
}

pub fn apply_round(state: &mut [BaseElement], op_code: u8, op_value: u8, step: usize) {
    // determine which round constants to use
    let ark = ARK[step % CYCLE_LENGTH];

    // apply first half of Rescue round
    apply_sbox(state);
    apply_mds(state);
    add_constants(state, &ark, 0);

    state[0] += BaseElement::from(op_code);
    state[1] += BaseElement::from(op_value);

    // apply second half of Rescue round
    apply_inv_sbox(state);
    apply_mds(state);
    add_constants(state, &ark, STATE_WIDTH);
}

// ROUND CONSTANTS
// ================================================================================================

/// Returns Rescue round constants arranged in column-major form.
pub fn get_round_constants() -> Vec<Vec<BaseElement>> {
    let mut constants = Vec::new();
    for _ in 0..(STATE_WIDTH * 2) {
        constants.push(vec![BaseElement::ZERO; CYCLE_LENGTH]);
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..CYCLE_LENGTH {
        for j in 0..(STATE_WIDTH * 2) {
            constants[j][i] = ARK[i][j];
        }
    }

    constants
}

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn add_constants(state: &mut [BaseElement], ark: &[BaseElement], offset: usize) {
    for i in 0..STATE_WIDTH {
        state[i] += ark[offset + i];
    }
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_sbox<E: FieldElement>(state: &mut [E]) {
    for i in 0..STATE_WIDTH {
        state[i] = state[i].exp(ALPHA.into());
    }
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_inv_sbox(state: &mut [BaseElement]) {
    for i in 0..STATE_WIDTH {
        state[i] = state[i].exp(INV_ALPHA);
    }
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_mds<E: FieldElement + From<BaseElement>>(state: &mut [E]) {
    let mut result = [E::ZERO; STATE_WIDTH];
    let mut temp = [E::ZERO; STATE_WIDTH];
    for i in 0..STATE_WIDTH {
        for j in 0..STATE_WIDTH {
            temp[j] = E::from(MDS[i * STATE_WIDTH + j]) * state[j];
        }

        for j in 0..STATE_WIDTH {
            result[i] += temp[j];
        }
    }
    state.copy_from_slice(&result);
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_inv_mds<E: FieldElement + From<BaseElement>>(state: &mut [E]) {
    let mut result = [E::ZERO; STATE_WIDTH];
    let mut temp = [E::ZERO; STATE_WIDTH];
    for i in 0..STATE_WIDTH {
        for j in 0..STATE_WIDTH {
            temp[j] = E::from(INV_MDS[i * STATE_WIDTH + j]) * state[j];
        }

        for j in 0..STATE_WIDTH {
            result[i] += temp[j];
        }
    }
    state.copy_from_slice(&result);
}

const ALPHA: u32 = 3;
const INV_ALPHA: u128 = 226854911280625642308916371969163307691;

const MDS: [BaseElement; STATE_WIDTH * STATE_WIDTH] = [
    BaseElement::new(340282366920938463463374557953744960808),
    BaseElement::new(1080),
    BaseElement::new(340282366920938463463374557953744961147),
    BaseElement::new(40),
    BaseElement::new(340282366920938463463374557953744932377),
    BaseElement::new(42471),
    BaseElement::new(340282366920938463463374557953744947017),
    BaseElement::new(1210),
    BaseElement::new(340282366920938463463374557953744079447),
    BaseElement::new(1277640),
    BaseElement::new(340282366920938463463374557953744532108),
    BaseElement::new(33880),
    BaseElement::new(340282366920938463463374557953720263017),
    BaseElement::new(35708310),
    BaseElement::new(340282366920938463463374557953733025977),
    BaseElement::new(925771),
];

const INV_MDS: [BaseElement; STATE_WIDTH * STATE_WIDTH] = [
    BaseElement::new(18020639985667067681479625318803400939),
    BaseElement::new(119196285838491236328880430704594968577),
    BaseElement::new(231409255903369280423951003551679307334),
    BaseElement::new(311938552114349342492438056332412246225),
    BaseElement::new(245698978747161380010236204726851770228),
    BaseElement::new(32113671753878130773768090116517402309),
    BaseElement::new(284248318938217584166130208504515171073),
    BaseElement::new(118503764402619831976614612559605579465),
    BaseElement::new(42476948408512208745085164298752800413),
    BaseElement::new(283594571303717652525183978492772054516),
    BaseElement::new(94047455979774690913009073579656179991),
    BaseElement::new(260445758149872374743470899536308888155),
    BaseElement::new(12603050626701424572717576220509072651),
    BaseElement::new(250660673575506110946271793719013778251),
    BaseElement::new(113894235293153614657151429548304212092),
    BaseElement::new(303406774346515776750608316419662860081),
];

pub const ARK: [[BaseElement; STATE_WIDTH * 2]; CYCLE_LENGTH] = [
    [
        BaseElement::new(252629594110556276281235816992330349983),
        BaseElement::new(121163867507455621442731872354015891839),
        BaseElement::new(244623479936175870778515556108748234900),
        BaseElement::new(181999122442017949289616572388308120964),
        BaseElement::new(130035663054758320517176088024859935575),
        BaseElement::new(274932696133623013607933255959111946013),
        BaseElement::new(130096286077538976127585373664362805864),
        BaseElement::new(209506446014122131232133742654202790201),
    ],
    [
        BaseElement::new(51912929769931267810162308005565017268),
        BaseElement::new(202610584823002946089528994694473145326),
        BaseElement::new(295992101426532309592836871256175669136),
        BaseElement::new(313404555247438968545340310449654540090),
        BaseElement::new(137671644572045862038757754124537020379),
        BaseElement::new(29113322527929260506148183779738829778),
        BaseElement::new(98634637270536166954048957710629281939),
        BaseElement::new(90484051915535813802492401077197602516),
    ],
    [
        BaseElement::new(193753019093186599897082621380539177732),
        BaseElement::new(88328997664086495053801384396180288832),
        BaseElement::new(134379598544046716907663161480793367313),
        BaseElement::new(50911186425769400405474055284903795891),
        BaseElement::new(12945394282446072785093894845750344239),
        BaseElement::new(110650301505380365788620562912149942995),
        BaseElement::new(154214463184362737046953674082326221874),
        BaseElement::new(306646039504788072647764955304698381135),
    ],
    [
        BaseElement::new(279745705918489041552127329708931301079),
        BaseElement::new(111293612078035530300709391234153848359),
        BaseElement::new(18110020378502034462498434861690576309),
        BaseElement::new(41797883582559360517115865611622162330),
        BaseElement::new(333888808893608021579859508112201825908),
        BaseElement::new(291192643991850989562610634125476905625),
        BaseElement::new(115042354025120848770557866862388897952),
        BaseElement::new(281483497320099569269754505499721335457),
    ],
    [
        BaseElement::new(172898111753678285350206449646444309824),
        BaseElement::new(202661860135906394577472615378659980424),
        BaseElement::new(141885268042225970011312316000526746741),
        BaseElement::new(270195331267041521741794476882482499817),
        BaseElement::new(196457080224171120865903216527675657315),
        BaseElement::new(56730777565482395039564396246195716949),
        BaseElement::new(4886253806084919544862202000090732791),
        BaseElement::new(147384194551383352824518757380733021990),
    ],
    [
        BaseElement::new(119476237236248181092343711369608370324),
        BaseElement::new(182869361251406039022577235058473348729),
        BaseElement::new(45308522364899994411952744852450066909),
        BaseElement::new(15438528253368638146901598290564135576),
        BaseElement::new(130060283207960095436997328133261743365),
        BaseElement::new(83953475955438079154228277940680487556),
        BaseElement::new(328659226769709797512044291035930357326),
        BaseElement::new(228749522131871685132212950281473676382),
    ],
    [
        BaseElement::new(46194972462682851176957413491161426658),
        BaseElement::new(296333983305826854863835978241833143471),
        BaseElement::new(138957733159616849361016139528307260698),
        BaseElement::new(67842086763518777676559492559456199109),
        BaseElement::new(45580040156133202522383315452912604930),
        BaseElement::new(67567837934606680937620346425373752595),
        BaseElement::new(202860989528104560171546683198384659325),
        BaseElement::new(22630500510153322451285114937258973361),
    ],
    [
        BaseElement::new(324160761097464842200838878419866223614),
        BaseElement::new(338466547889555546143667391979278153877),
        BaseElement::new(189171173535649401433078628567098769571),
        BaseElement::new(162173266902020502126600904559755837464),
        BaseElement::new(136209703129442038834374731074825683052),
        BaseElement::new(61998071517031804812562190829480056772),
        BaseElement::new(307309080039351604461536918194634835054),
        BaseElement::new(26708622949278137915061761772299784349),
    ],
    [
        BaseElement::new(129516553661717764361826568456881002617),
        BaseElement::new(224023580754958002183324313900177991825),
        BaseElement::new(17590440203644538688189654586240082513),
        BaseElement::new(135610063062379124269847491297867667710),
        BaseElement::new(146865534517067293442442506551295645352),
        BaseElement::new(238139104484181583196227119098779158429),
        BaseElement::new(39300761479713744892853256947725570060),
        BaseElement::new(54114440355764484955231402374312070440),
    ],
    [
        BaseElement::new(222758070305343916663075833184045878425),
        BaseElement::new(323840793618712078836672915700599856701),
        BaseElement::new(103586087979277053032666296091805459741),
        BaseElement::new(160263698024385270625527195046420579470),
        BaseElement::new(76620453913654705501329735586535761337),
        BaseElement::new(117793948142462197480091377165008040465),
        BaseElement::new(86998218841589258723143213495722487114),
        BaseElement::new(203188618662906890442620821687773659689),
    ],
    [
        BaseElement::new(313098786815741054633864043424353402357),
        BaseElement::new(133085673687338880872979866135939079867),
        BaseElement::new(219888424885634764555580944265544343421),
        BaseElement::new(5893221169005427793512575133564978746),
        BaseElement::new(123830602624063632344313821515642988189),
        BaseElement::new(99030942908036387138287682010525589136),
        BaseElement::new(181549003357535890945363082242256699137),
        BaseElement::new(152424978799328476472358562493335008209),
    ],
    [
        BaseElement::new(274481943862544603168725464029979191673),
        BaseElement::new(4975004592976331754728718693838357226),
        BaseElement::new(101850445399221640701542169338886750079),
        BaseElement::new(230325699922192981509673754024218912397),
        BaseElement::new(50419227750575087142720761582056939006),
        BaseElement::new(112444234528764731925178653200320603078),
        BaseElement::new(312169855609816651638877239277948636598),
        BaseElement::new(204255114617024487729019111502542629940),
    ],
    [
        BaseElement::new(95797476952346525817251811755749179939),
        BaseElement::new(306977388944722094681694167558392710189),
        BaseElement::new(300754874465668732709232449646112602172),
        BaseElement::new(25567836410351071106804347269705784680),
        BaseElement::new(129659188855548935155840545784705385753),
        BaseElement::new(228441586459539470069565041053012869566),
        BaseElement::new(178382533299631576605259357906020320778),
        BaseElement::new(274458637266680353971597477639962034316),
    ],
    [
        BaseElement::new(280059913840028448065185235205261648486),
        BaseElement::new(246537412674731137211182698562269717969),
        BaseElement::new(259930078572522349821084822750913159564),
        BaseElement::new(186061633995391650657311511040160727356),
        BaseElement::new(179777566992900315528995607912777709520),
        BaseElement::new(209753365793154515863736129686836743468),
        BaseElement::new(270445008049478596978645420017585428243),
        BaseElement::new(70998387591825316724846035292940615733),
    ],
    [BaseElement::ZERO; 8],
    [BaseElement::ZERO; 8],
];

#[test]
fn test() {
    let mut state0 = vec![BaseElement::ZERO; 4];

    apply_round(&mut state0,0u8, 0u8, 0);

    println!("{:?}", state0);
}