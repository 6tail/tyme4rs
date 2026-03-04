use crate::tyme::culture::{Element, Zodiac, ELEMENT_NAMES, ZODIAC_NAMES};
use crate::tyme::sixtycycle::SixtyCycle;
use crate::tyme::solar::{SolarDay, SolarYear};
use crate::tyme::unit::{DayUnit, MonthUnit};
use crate::tyme::{Culture, Tyme};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::{Chars, Split};

/// 藏历五行
#[derive(Debug, Clone)]
pub struct RabByungElement {
    parent: Element,
}

impl Deref for RabByungElement {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for RabByungElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Tyme for RabByungElement {
    fn next(&self, n: isize) -> Self {
        Self {
            parent: Element::from_index(self.get_index() as isize + n),
        }
    }
}

impl Culture for RabByungElement {
    fn get_name(&self) -> String {
        self.parent.get_name().replace("金", "铁")
    }
}

impl RabByungElement {
    pub fn from_index(index: isize) -> Self {
        Self {
            parent: Element::from_index(index),
        }
    }

    pub fn from_name(name: &str) -> Self {
        Self {
            parent: Element::from_name(&name.replace("铁", "金")),
        }
    }

    /// 我生者
    pub fn get_reinforce(&self) -> Self {
        self.next(1)
    }

    /// 我克者
    pub fn get_restrain(&self) -> Self {
        self.next(2)
    }

    /// 生我者
    pub fn get_reinforced(&self) -> Self {
        self.next(-1)
    }

    /// 克我者
    pub fn get_restrained(&self) -> Self {
        self.next(-2)
    }
}

impl Display for RabByungElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl PartialEq for RabByungElement {
    fn eq(&self, other: &Self) -> bool {
        self.get_index() == other.get_index()
    }
}

impl Eq for RabByungElement {}

impl From<RabByungElement> for Element {
    fn from(val: RabByungElement) -> Self {
        val.parent
    }
}

/// 藏历年 (饶迥年)
#[derive(Debug, Clone)]
pub struct RabByungYear {
    /// 饶迥(胜生周)序号，从0开始
    rab_byung_index: usize,
    /// 五行索引，从0开始
    element_index: usize,
    /// 生肖索引，从0开始
    zodiac_index: usize,
}

impl Culture for RabByungYear {
    fn get_name(&self) -> String {
        let digits: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
        let units: [&str; 3] = ["", "十", "百"];
        let mut n: usize = self.rab_byung_index + 1;
        let mut s: String = String::new();
        let mut pos: usize = 0;

        while n > 0 {
            let digit: usize = n % 10;
            if digit > 0 {
                s = format!("{}{}{}", digits[digit], units[pos], s);
            } else if !s.is_empty() {
                s = format!("{}{}", digits[digit], s);
            }
            n /= 10;
            pos += 1;
        }
        if s.starts_with("一十") {
            s = s.chars().skip(1).collect();
        }
        format!(
            "第{}饶迥{}{}年",
            s,
            self.get_element().get_name(),
            self.get_zodiac().get_name()
        )
    }
}

impl RabByungYear {
    pub fn new(
        rab_byung_index: isize,
        element_index: usize,
        zodiac_index: usize,
    ) -> Result<Self, String> {
        if !(0..=150).contains(&rab_byung_index) {
            return Err(format!("illegal rab-byung index: {}", rab_byung_index));
        }
        if element_index >= ELEMENT_NAMES.len() {
            return Err(format!("illegal relement index: {}", element_index));
        }
        if zodiac_index >= ZODIAC_NAMES.len() {
            return Err(format!("illegal zodiac index: {}", zodiac_index));
        }
        Ok(Self {
            rab_byung_index: rab_byung_index as usize,
            element_index,
            zodiac_index,
        })
    }

    pub fn validate(year: isize) -> Result<(), String> {
        if !(1027..=9999).contains(&year) {
            Err(format!("illegal rab-byung year: {}", year))
        } else {
            Ok(())
        }
    }

    /// 从饶迥序号和六十甲子创建
    pub fn from_sixty_cycle(
        rab_byung_index: isize,
        sixty_cycle: SixtyCycle,
    ) -> Result<Self, String> {
        Self::new(
            rab_byung_index,
            sixty_cycle.get_heaven_stem().get_element().get_index(),
            sixty_cycle.get_earth_branch().get_zodiac().get_index(),
        )
    }

    /// 从五行和生肖创建
    pub fn from_element_zodiac(
        rab_byung_index: isize,
        element: RabByungElement,
        zodiac: Zodiac,
    ) -> Result<Self, String> {
        Self::new(rab_byung_index, element.get_index(), zodiac.get_index())
    }

    /// 从公历年创建 (1027年为藏历元年)
    pub fn from_year(year: isize) -> Result<Self, String> {
        Self::validate(year)?;
        Self::from_sixty_cycle((year - 1024) / 60, SixtyCycle::from_index(year - 4))
    }

    /// 饶迥序号
    pub fn get_rab_byung_index(&self) -> usize {
        self.rab_byung_index
    }

    /// 干支
    pub fn get_sixty_cycle(&self) -> SixtyCycle {
        let z: isize = self.zodiac_index as isize;
        SixtyCycle::from_index(6 * (self.element_index as isize * 2 + z % 2) - 5 * z)
    }

    /// 生肖
    pub fn get_zodiac(&self) -> Zodiac {
        Zodiac::from_index(self.zodiac_index as isize)
    }

    /// 藏历五行
    pub fn get_element(&self) -> RabByungElement {
        RabByungElement::from_index(self.element_index as isize)
    }

    /// 年
    pub fn get_year(&self) -> isize {
        1024 + self.rab_byung_index as isize * 60 + self.get_sixty_cycle().get_index() as isize
    }

    /// 闰月数字，1代表闰1月，0代表无闰月
    pub fn get_leap_month(&self) -> usize {
        let mut y: isize = 1;
        let mut m: isize = 4;
        let mut t: isize = 1;
        let current_year: isize = self.get_year();

        while y < current_year {
            let i: isize = m + 31 + t;
            y += 2;
            m = i - 23;
            if i > 35 {
                y += 1;
                m -= 12;
            }
            t = 1 - t;
        }

        if y == current_year {
            m as usize
        } else {
            0
        }
    }

    pub fn next(&self, n: isize) -> Result<Self, String> {
        Self::from_year(self.get_year() + n)
    }

    /// 公历年
    pub fn get_solar_year(&self) -> SolarYear {
        SolarYear::from_year(self.get_year())
    }

    /// 月份数量
    pub fn get_month_count(&self) -> usize {
        let mut n: usize = 12;
        if self.get_leap_month() > 0 {
            n = 13
        }
        n
    }

    /// 首月
    pub fn get_first_month(&self) -> RabByungMonth {
        RabByungMonth::from_ym(self.get_year(), 1).unwrap()
    }

    pub fn get_months(&self) -> Vec<RabByungMonth> {
        let mut l: Vec<RabByungMonth> = Vec::new();
        let y: isize = self.get_year();
        let leap_month: isize = self.get_leap_month() as isize;
        for i in 1..13 {
            l.push(RabByungMonth::from_ym(y, i).unwrap());
            if i == leap_month {
                l.push(RabByungMonth::from_ym(y, -i).unwrap());
            }
        }
        l
    }
}

impl Display for RabByungYear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl PartialEq for RabByungYear {
    fn eq(&self, other: &Self) -> bool {
        self.get_rab_byung_index() == other.get_rab_byung_index()
            && self.get_sixty_cycle().get_index() == other.get_sixty_cycle().get_index()
    }
}

impl Eq for RabByungYear {}

lazy_static! {
    static ref RAB_BYUNG_MONTH_DAYS: HashMap<usize, Vec<isize>> = {
        let mut map: HashMap<usize, Vec<isize>> = HashMap::new();
        let years: Split<&str> = "2c>,182[>1:2TA4ZI=n1E2Bk1J2Ff3Mk503Oc62g=,172^>1:2XA1>2UE2Bo1I2Fj3Lo62Fb3Mf5,03N^72b=1:2]A1>2ZF1B2VI2Em1K2Fe,2Lh1R3Na603P\\:172Y>1;2UB2=m2Dq1J2Eh,2Kl1Q3Me603Pa:172^>1;2YA2=p1C2UI,2Dk2Jp3QEc3Mi603Pf:3L[72b?1:2]A1<2UB2XH,2Cn1I2Ei1L2Ie1Q3Na703Q\\:2`@1;2XA,4\\H;m1B2TI2Em1L2Ij1Q3Nf603Q`903QW:,2[@1;2TB2XI1E4TMAh2Io3RFe3Mj603Pc803Q[;,2^?1;2WA2>q1E2Bm1I2Fi1M2Hc3Of70,3P^82a>1:2[A1>2WE1B2TI2Fm1L2Hf3Ni6,03Oa703PZ:3`A62V>4]F;q1B4YJ>l2Eq1L2Gi3Ml5,03Nd603Q_9172[>1;2XB2>p1E2VK2Fl,1K2Fc3Mh603Pc9172`>1;2\\B1>2UD2=j2En,1J2Fg3Mm62Ib3Pj;3M_703R[:2`B1=2YB2=n,1C2TI2Fk1L2Ig1P3Nd703Q_:152X<2[A,2<q1B2WI2Ep1L2Il1Q3Ni703Qc9152[:2^@,1;2WB2>o1E2Bk1I2Fh1M2Ib3Pf803R^9,2a?1;2ZA1>2UE2Bp1I2Fl1M2If3Oi80,3Pa803QY:2^A1>2ZE1B4WJ>j2Fp1M2Hi1N2H`,3Od703Q]:162Y>1;2VB2?o1E4VM@h2Gl1M,2Hd3Ng603Qa9172^>1;2ZB1?2UE2@l2Fo1L,2Gg3Mk62H`3Pf:172c?3QY;2_B1>2YD2?o1E,2TK2Fj1M2Ie1P3Mb703R^;172X=2\\C1>,2TD2WJ2Fn1L2Ij1P3Ng703Rb:162[<2_B1=,2VC2>m1E4TMAh2Io3QFe3Nl82Ja3Qf:152_;0,3RU<2ZB1>2TE2Bn1I2Fj1M2Je3Pk:2K^3Ra:,03RY;2]A1>2XE1B2TI2Fo1M2Ii1P2Ka3Qd8,03R]:3bB62W>4]F:q1B2?n1F4VNAh2Il1O2Jd,3Pg803Q`:162\\=1;2XB1?2TF2Bl2Ho1N,2Ig3Nk703Qd9162`>1;2]B1?2XE2Ao1G2TM,2Hj1M2Id1P3M_603R\\;172W>2\\E1@2TE,2?i2Gm1M2Ih1P3Md603Ra;172[=28q1?2WD,2?m2Fq1M2Il1P3Mi72I^3Re:162_<172W=,2ZC2?q1E2Bk1I2Fh1M2Jd1Q3M^52b;16,2Y<2]B1>2VE2Bp1I2Fm1M2Jh1Q2Lb3Re:15,2\\;3aC62U>2[E1B4WJ>k1F4TNBg2Jl1P2Le3Qh9,03R`:172Z=1:2VB2?q1F2Bk2Ip1P2Jg,1P2J_3Qc:162^=1;2[B1?2WF2Bo1H2Bg2Ij,1O2Jc3Qg:3L\\62c>3QY;3aC72V?2[F1A2TG2Bj,2Hm1N2Jg1P3Mb603R_;182Z>1:2T@2WF2Am,2Gp1M2Ik1P3Mg603Rc;172^>192W?2ZE,2@p1F2Bj2Io3QEe1M2Jb1Q3M]72b=182Z>,2]D1?2VE2Bn1I2Fk1M2Jg1Q3Ma62e<172]=,172U>2YE1B2UI2Fp1N2Jk1Q3Me503M\\6,2`<172Y>3_F:2TB2?n1F2Cj2Jo3QDc2Lh1R,3L_52c;172]=1:2XB1?2UF2Cn1I2Eg2Kk1P,2Lb3Rf;162a=1:2]B1?2ZF1B2TH2Dj2Jm,1O2Kf1Q3M`603Q\\;182Y?2;q1A2WH2Cm,2Hq1O2Ji1P3Me603Qa;182]>1:2WA2[G2Ap,1G2Bi2Im1P3Mi72I_3Qf;3N\\72Eh1:2Z?29o,1@2UF2Bm1I2Fh1M2Je1Q3N`72f?3PY92]>19,2U?2YF2Bq1I2Fm1M2Jj1Q3Nd603O]72`=,182X?4]F:o1B4WI=k1F4UNCi2Jn3REc3Mh503N`6,2c<182\\>1:2VA2?q1F2Cm1J2Fg2Lk1R3Mc5,2f<172`=1:2[A1?2XF2Cq1I2Ek2Kn1R,2Lf1R3N_62d>3PZ:3aC72W?2;p1B2WI2Dn1J,2De2Ki1Q3Mc603Q_:182\\?1;2VB2<m2Cq1I,2Dh2Jl1P3Mg603Qd;182`?1;2ZA2<p1B,2UH2Cl1I2Ef3Mm82Jc1Q3N_703QY:2]@1;2UA,2XG2Bp1I2Fk1M2Jh1Q3Nc703Q]92`?1:,2X@4\\G:n1B2VI2Fp1M2Jl1R3Ng603P`82d>,192[?1;2UA2>o1F2Ck1J2Gg3Mk603Oc70,3OZ82_>1:2YA1?2VF2Cp1J2Fj1M2Gc3Nf5,03O^72b>1:2^B1?4[G;n1C2VJ2Fn1L2Gf,3Mi503Nb603Q]:172Y?1<2UB2>m2Eq1K2Fi,2Kl1R3Mf603Qa:182^?1;2YB2>q1D2VJ,2Dl1J2Fe3Mj603Qg;3N]72c@3QX;2]A1=2VB,2YI2Co1J2Fi1M2Je1Q3Nb703R]:2aA1<2XA,2<n1C2UI2Fn1M2Jj1Q3Nf703Q`903RX:,2[@1<2TB4YJ>l1E4UNBi1J2Ge3Mk703Pc803Q[9,2^?1;2XB2>q1E2Cn1J2Gj1M2Ic3Of70,3P^82b?1;2\\A1>2XF1C2UJ2Fm1M2Hf3Ni6,03Oa703Q[:3aB72W>1<2TC2?m2Fq1L2Gi3Ml5,03Ne703Q_:172\\>1<2XB2?q1E2WL2Fl,1L2Gd3Ni603Qd:172a?1;2\\B1>2VD2>k,2Eo1K2Gh1M2Ic1Q3N`703R\\;3aC62U=2YC2>o,1D2TJ2Fl1M2Jh1Q3Ne703R`:162Y<2\\B,1=2TC4XJ=j2Fp1M2Jm3QFc3Ni803Qc:152\\;2_A,1<2WB2>o1E2Bl1J2Gh1N2Jc3Qg903R^:,2b@1;2[B1>2VE2Cq1J2Gl1N2Jf3Pj80,3Qa803RZ;2_B1>4[F:o1C4XK?k2Fp1M2Ii1O2Ia,3Pd703R^:172Y>1<2VC2?p1F2Ai2Hl1M,2Hd3Oh703Qb:172^>1<2[C1?2UE2Al2Go,1L2Hg3Nl82Ia3Qg;3M]72e@3RZ;3`C72T>2YD2@o1E,2TK2Gk1M2Jf1Q3Nb703R^;172Y=2\\D1>,2TD4XK>i2Fo1M2Jj1Q3Ng703Rb;172\\<2`C1=,2WC2?n1F4VNBi1J2Gf1N2Kb3Rf:162_;15,2V<2ZB1?2TE2Bn1J2Gk1N2Kf1Q2L^3Rb:,152Z;2^B1>2YE1B2UJ2Go1N2Ji1P2Kb3Qd9,03R];172X>1;2TC2@n1G2Bi2Im1O2Jd,3Ph803Ra:172\\>1;2YC1@2UF2Bl2Hp1N,2Ig3Ol82J`3Qe:172a>1;4^C7q1?2XF2Ao1G2UN,2Hj1N2Jd1Q3N`703R];182X>2]F1@2TF,2@j2Gn1M2Jq1Q3Ne703Ra;172\\>192T?,2WE2@m1F4TMAf2Im3QEc3Nj82J`3Rf;172_=182W>,2ZD2?q1F2Bl1I2Gj1N2Ke1R3M_62b<17,2Z=2]C1?2WE2Bq1I2Gn1N2Ki1Q3Mb52e;16,2]<172V>4[F:o1B4XK?l1G4UOCh2Jl1Q2Le3Rh:,152`;172Z>1;2WB2@q1G2Cl2Ip1P2K_".split(",");

        let mut y: usize = 1950;
        let mut m: usize = 11;

        for s in years {
            let mut ys: &str = s;
            while !ys.is_empty() {
                let mut chars: Chars = ys.chars();
                let len: usize = (chars.next().unwrap() as isize - b'0' as isize) as usize;
                let mut data: Vec<isize> = Vec::new();
                for _i in 0..len {
                    data.push(chars.next().unwrap() as isize - b'5' as isize - 30);
                }
                map.insert(y * 13 + m, data);
                m += 1;
                ys = &ys[1 + len..];
            }
            y += 1;
            m = 0;
        }
        map
    };
}

/// 藏历月
#[derive(Debug, Clone)]
pub struct RabByungMonth {
    parent: MonthUnit,
    /// 是否闰月
    leap: bool,
}

impl Deref for RabByungMonth {
    type Target = MonthUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for RabByungMonth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Culture for RabByungMonth {
    fn get_name(&self) -> String {
        let name: &str = Self::NAMES[self.get_month() - 1];
        if self.leap {
            format!("闰{}", name)
        } else {
            name.to_string()
        }
    }
}

impl RabByungMonth {
    const NAMES: [&'static str; 12] = [
        "正月",
        "二月",
        "三月",
        "四月",
        "五月",
        "六月",
        "七月",
        "八月",
        "九月",
        "十月",
        "十一月",
        "十二月",
    ];
    const ALIAS: [&'static str; 12] = [
        "神变月",
        "苦行月",
        "具香月",
        "萨嘎月",
        "作净月",
        "明净月",
        "具醉月",
        "具贤月",
        "天降月",
        "持众月",
        "庄严月",
        "满意月",
    ];

    /// 从藏历年月创建
    pub fn new(year: isize, month: isize) -> Result<Self, String> {
        Self::validate(year, month)?;
        Ok(Self {
            parent: MonthUnit::new(year, month.abs()),
            leap: month < 0,
        })
    }

    pub fn validate(year: isize, month: isize) -> Result<(), String> {
        if month == 0 || !(-12..=12).contains(&month) {
            return Err(format!("illegal rab-byung month: {}", month));
        }

        if !(1950..=2050).contains(&year) {
            return Err(format!(
                "rab-byung year {} must between 1950 and 2050",
                year
            ));
        }

        let leap: bool = month < 0;
        let m: usize = month.unsigned_abs();
        if year == 1950 && m < 12 {
            return Err(format!(
                "month {} must be 12 in rab-byung year {}",
                month, year
            ));
        }

        if leap && m != RabByungYear::from_year(year)?.get_leap_month() {
            return Err(format!(
                "illegal leap month {} in rab-byung year {}",
                m, year
            ));
        }

        Ok(())
    }

    pub fn from_ym(year: isize, month: isize) -> Result<Self, String> {
        Self::new(year, month)
    }

    /// 藏历年
    pub fn get_rab_byung_year(&self) -> RabByungYear {
        RabByungYear::from_year(self.get_year()).unwrap()
    }

    pub fn get_month(&self) -> usize {
        self.parent.get_month() as usize
    }

    /// 带闰信息的月份 (负数为闰月)
    pub fn get_month_with_leap(&self) -> isize {
        match self.leap {
            false => self.get_month() as isize,
            _ => -(self.get_month() as isize),
        }
    }

    /// 在年中的索引 (0-12)
    pub fn get_index_in_year(&self) -> usize {
        let m: usize = self.get_month();
        let mut index: usize = m - 1;
        if self.leap {
            index += 1;
        } else {
            let leap_month: usize = self.get_rab_byung_year().get_leap_month();
            if leap_month > 0 && m > leap_month {
                index += 1;
            }
        }
        index
    }

    /// 是否闰月
    pub fn is_leap(&self) -> bool {
        self.leap
    }

    /// 别名 (如 "闰神变月")
    pub fn alias(&self) -> String {
        let alias: &str = Self::ALIAS[self.get_month() - 1];
        if self.leap {
            format!("闰{}", alias)
        } else {
            alias.to_string()
        }
    }

    /// 下一月
    pub fn next(&self, n: isize) -> Result<Self, String> {
        if n == 0 {
            return Ok(self.clone());
        }

        let mut m: isize = self.get_index_in_year() as isize + 1 + n;
        let mut y: RabByungYear = self.get_rab_byung_year();
        if n > 0 {
            let mut month_count: isize = y.get_month_count() as isize;
            while m > month_count {
                m -= month_count;
                y = y.next(1)?;
                month_count = y.get_month_count() as isize;
            }
        } else {
            while m <= 0 {
                y = y.next(-1)?;
                m += y.get_month_count() as isize;
            }
        }

        let mut leap: bool = false;
        let leap_month: isize = y.get_leap_month() as isize;
        if leap_month > 0 {
            if m == leap_month + 1 {
                leap = true;
            }
            if m > leap_month {
                m -= 1;
            }
        }

        Self::new(y.get_year(), if leap { -m } else { m })
    }

    /// 特殊日子列表 (闰日为正，缺日为负)
    pub fn get_special_days(&self) -> Vec<isize> {
        let key: usize = self.get_year() as usize * 13 + self.get_index_in_year();
        RAB_BYUNG_MONTH_DAYS.get(&key).cloned().unwrap_or_default()
    }

    /// 闰日列表
    pub fn get_leap_days(&self) -> Vec<isize> {
        self.get_special_days()
            .iter()
            .filter(|&&d| d > 0)
            .copied()
            .collect()
    }

    /// 缺日列表
    pub fn get_miss_days(&self) -> Vec<isize> {
        self.get_special_days()
            .iter()
            .filter(|&&d| d < 0)
            .copied()
            .collect()
    }

    /// 当月天数
    pub fn get_day_count(&self) -> usize {
        30 + self.get_leap_days().len() - self.get_miss_days().len()
    }

    /// 首日
    pub fn get_first_day(&self) -> RabByungDay {
        RabByungDay::new(self.get_year(), self.get_month_with_leap(), 1).unwrap()
    }

    pub fn get_days(&self) -> Vec<RabByungDay> {
        let mut l: Vec<RabByungDay> = Vec::new();
        let miss_days: Vec<isize> = self.get_miss_days();
        let leap_days: Vec<isize> = self.get_leap_days();
        let y: isize = self.get_year();
        let m: isize = self.get_month_with_leap();
        for i in 1..31 {
            if miss_days.contains(&i) {
                continue;
            }
            l.push(RabByungDay::new(y, m, i).unwrap());
            if leap_days.contains(&i) {
                l.push(RabByungDay::new(y, m, -i).unwrap());
            }
        }
        l
    }
}

impl Display for RabByungMonth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_rab_byung_year(), self.get_name())
    }
}

impl PartialEq for RabByungMonth {
    fn eq(&self, other: &Self) -> bool {
        self.get_year() == other.get_year()
            && self.get_month_with_leap() == other.get_month_with_leap()
    }
}

impl Eq for RabByungMonth {}

/// 藏历日
#[derive(Debug, Clone)]
pub struct RabByungDay {
    parent: DayUnit,
    /// 是否闰日
    leap: bool,
}

impl Deref for RabByungDay {
    type Target = DayUnit;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for RabByungDay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl Culture for RabByungDay {
    fn get_name(&self) -> String {
        let name: &str = Self::NAMES[self.get_day() - 1];
        if self.leap {
            format!("闰{}", name)
        } else {
            name.to_string()
        }
    }
}

impl RabByungDay {
    const NAMES: [&'static str; 30] = [
        "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一",
        "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二",
        "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
    ];

    /// 创建藏历日
    pub fn new(year: isize, month: isize, day: isize) -> Result<Self, String> {
        Self::validate(year, month, day)?;
        Ok(Self {
            parent: DayUnit::new(year, month, day.abs()),
            leap: day < 0,
        })
    }

    pub fn validate(year: isize, month: isize, day: isize) -> Result<(), String> {
        if day == 0 || !(-30..=30).contains(&day) {
            return Err(format!("illegal day {} in {}", day, month));
        }

        let m: RabByungMonth = RabByungMonth::from_ym(year, month)?;
        let leap: bool = day < 0;
        let d: isize = day.abs();

        if leap && !m.get_leap_days().contains(&d) {
            return Err(format!("illegal leap day {} in {}", d, month));
        }

        if !leap && m.get_miss_days().contains(&d) {
            return Err(format!("illegal day {} in {}", d, month));
        }

        Ok(())
    }

    pub fn from_ymd(year: isize, month: isize, day: isize) -> Result<Self, String> {
        Self::new(year, month, day)
    }

    pub fn from_solar_day(solar_day: SolarDay) -> Result<Self, String> {
        let mut days: isize = solar_day.subtract(SolarDay::from_ymd(1951, 1, 8));
        let mut m: RabByungMonth = RabByungMonth::from_ym(1950, 12)?;
        let mut count: isize = m.get_day_count() as isize;
        while days >= count {
            days -= count;
            m = m.next(1)?;
            count = m.get_day_count() as isize;
        }
        let mut day: isize = days + 1;
        for &d in m.get_special_days().iter() {
            if d < 0 {
                if day >= -d {
                    day += 1;
                }
            } else if d > 0 {
                if day == d + 1 {
                    day = -d;
                    break;
                } else if day > d + 1 {
                    day -= 1;
                }
            }
        }
        Self::new(m.get_year(), m.get_month_with_leap(), day)
    }

    pub fn get_rab_byung_month(&self) -> RabByungMonth {
        RabByungMonth::from_ym(self.get_year(), self.get_month()).unwrap()
    }

    pub fn get_day(&self) -> usize {
        self.parent.get_day() as usize
    }

    pub fn is_leap(&self) -> bool {
        self.leap
    }

    /// 带闰信息的日期 (负数为闰日)
    pub fn get_day_with_leap(&self) -> isize {
        match self.leap {
            false => self.get_day() as isize,
            _ => -(self.get_day() as isize),
        }
    }

    pub fn next(&self, n: isize) -> Result<Self, String> {
        if n == 0 {
            Ok(self.clone())
        } else {
            self.get_solar_day().next(n).get_rab_byung_day()
        }
    }

    /// 转换为公历日
    pub fn get_solar_day(&self) -> SolarDay {
        let mut m: RabByungMonth = RabByungMonth::from_ym(1950, 12).unwrap();
        let cm: RabByungMonth = self.get_rab_byung_month();
        let mut n: isize = 0;
        while m != cm {
            n += m.get_day_count() as isize;
            m = m.next(1).unwrap();
        }
        let mut t: isize = self.get_day() as isize;
        for &d in m.get_special_days().iter() {
            if d < 0 {
                if t > -d {
                    t -= 1;
                }
            } else if d > 0 && t > d {
                t += 1;
            }
        }
        if self.leap {
            t += 1;
        }
        SolarDay::from_ymd(1951, 1, 7).next(n + t)
    }

    pub fn subtract(&self, other: Self) -> isize {
        self.get_solar_day().subtract(other.get_solar_day())
    }
}

impl Display for RabByungDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_rab_byung_month(), self.get_name())
    }
}

impl PartialEq for RabByungDay {
    fn eq(&self, other: &Self) -> bool {
        self.get_month() == other.get_month()
            && self.get_day_with_leap() == other.get_day_with_leap()
    }
}

impl Eq for RabByungDay {}

#[cfg(test)]
mod tests {
    use crate::tyme::culture::Zodiac;
    use crate::tyme::rabbyung::{RabByungDay, RabByungElement, RabByungMonth, RabByungYear};
    use crate::tyme::solar::SolarDay;
    use crate::tyme::Culture;

    #[test]
    fn test0() {
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            0,
            RabByungElement::from_name("火"),
            Zodiac::from_name("兔"),
        )
        .unwrap();
        assert_eq!("第一饶迥火兔年", y.get_name());
        assert_eq!("1027年", y.get_solar_year().get_name());
        assert_eq!("丁卯", y.get_sixty_cycle().get_name());
        assert_eq!(10, y.get_leap_month());
    }

    #[test]
    fn test1() {
        assert_eq!(
            "第一饶迥火兔年",
            RabByungYear::from_year(1027).unwrap().get_name()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            "第十七饶迥铁虎年",
            RabByungYear::from_year(2010).unwrap().get_name()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(5, RabByungYear::from_year(2043).unwrap().get_leap_month());
        assert_eq!(0, RabByungYear::from_year(2044).unwrap().get_leap_month());
    }

    #[test]
    fn test4() {
        assert_eq!(
            "第十六饶迥铁牛年",
            RabByungYear::from_year(1961).unwrap().get_name()
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            "第十六饶迥铁虎年十二月",
            RabByungMonth::from_ym(1950, 12).unwrap().to_string()
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            "第十六饶迥铁虎年十二月初一",
            SolarDay::from_ymd(1951, 1, 8)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            15,
            RabByungElement::from_name("铁"),
            Zodiac::from_name("虎"),
        )
        .unwrap();
        assert_eq!(
            "1951年1月8日",
            RabByungDay::from_ymd(y.get_year(), 12, 1)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            "第十八饶迥铁马年十二月三十",
            SolarDay::from_ymd(2051, 2, 11)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            17,
            RabByungElement::from_name("铁"),
            Zodiac::from_name("马"),
        )
        .unwrap();
        assert_eq!(
            "2051年2月11日",
            RabByungDay::from_ymd(y.get_year(), 12, 30)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            "第十七饶迥木蛇年二月廿五",
            SolarDay::from_ymd(2025, 4, 23)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            16,
            RabByungElement::from_name("木"),
            Zodiac::from_name("蛇"),
        )
        .unwrap();
        assert_eq!(
            "2025年4月23日",
            RabByungDay::from_ymd(y.get_year(), 2, 25)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test9() {
        assert_eq!(
            "第十六饶迥铁兔年正月初二",
            SolarDay::from_ymd(1951, 2, 8)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            15,
            RabByungElement::from_name("铁"),
            Zodiac::from_name("兔"),
        )
        .unwrap();
        assert_eq!(
            "1951年2月8日",
            RabByungDay::from_ymd(y.get_year(), 1, 2)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test10() {
        assert_eq!(
            "第十六饶迥铁虎年十二月闰十六",
            SolarDay::from_ymd(1951, 1, 24)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            15,
            RabByungElement::from_name("铁"),
            Zodiac::from_name("虎"),
        )
        .unwrap();
        assert_eq!(
            "1951年1月24日",
            RabByungDay::from_ymd(y.get_year(), 12, -16)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test11() {
        assert_eq!(
            "第十六饶迥铁牛年五月十一",
            SolarDay::from_ymd(1961, 6, 24)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            15,
            RabByungElement::from_name("铁"),
            Zodiac::from_name("牛"),
        )
        .unwrap();
        assert_eq!(
            "1961年6月24日",
            RabByungDay::from_ymd(y.get_year(), 5, 11)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test12() {
        assert_eq!(
            "第十六饶迥铁兔年十二月廿八",
            SolarDay::from_ymd(1952, 2, 23)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
        let y: RabByungYear = RabByungYear::from_element_zodiac(
            15,
            RabByungElement::from_name("铁"),
            Zodiac::from_name("兔"),
        )
        .unwrap();
        assert_eq!(
            "1952年2月23日",
            RabByungDay::from_ymd(y.get_year(), 12, 28)
                .unwrap()
                .get_solar_day()
                .to_string()
        );
    }

    #[test]
    fn test13() {
        assert_eq!(
            "第十七饶迥木蛇年二月廿九",
            SolarDay::from_ymd(2025, 4, 26)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn test14() {
        assert_eq!(
            "第十七饶迥木蛇年二月廿七",
            SolarDay::from_ymd(2025, 4, 25)
                .get_rab_byung_day()
                .unwrap()
                .to_string()
        );
    }
}
