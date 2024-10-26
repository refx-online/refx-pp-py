use refx_pp::{
    osu::{OsuDifficultyAttributes, OsuPerformanceAttributes}, osu_2019::OsuPP, osu_2019_2::FxPP, AnyPP, AnyStars, DifficultyAttributes, GameMode, Mods, PerformanceAttributes
};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    pyclass, pymethods,
    types::PyDict,
    PyResult,
};

use crate::{
    beatmap::PyBeatmap, diff_attrs::PyDifficultyAttributes, error::KwargsError,
    map_attrs::PyBeatmapAttributes, perf_attrs::PyPerformanceAttributes, strains::PyStrains,
};

#[pyclass(name = "Calculator")]
#[derive(Default)]
pub struct PyCalculator {
    attributes: Option<DifficultyAttributes>,
    mode: Option<GameMode>,
    mods: Option<u32>,
    acc: Option<f64>,
    n_geki: Option<usize>,
    n_katu: Option<usize>,
    n300: Option<usize>,
    n100: Option<usize>,
    n50: Option<usize>,
    n_misses: Option<usize>,
    combo: Option<usize>,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
    shaymi_mode: bool,

    ac: Option<usize>,
    arc: Option<f64>,
    hdr: Option<bool>,

    tw: Option<usize>,
    cs: Option<bool>,
    notrefx: bool, // not needed? actually just incase
}

macro_rules! set_calc {
    ( $calc:ident, $this:ident: $( $field:ident ,)* ) => {
        $(
            if let Some(val) = $this.$field {
                $calc = $calc.$field(val);
            }
        )*
    };
}

#[pymethods]
impl PyCalculator {
    #[new]
    #[args(kwargs = "**")]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let kwargs = match kwargs {
            Some(kwargs) => kwargs,
            None => return Ok(Self::default()),
        };

        let mut this = Self::default();

        for (key, value) in kwargs.iter() {
            match key.extract()? {
                "mode" => {
                    let int = value
                        .extract::<u8>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'mode': must be an int"))?;

                    this.mode = match int {
                        0 => Some(GameMode::Osu),
                        1 => Some(GameMode::Taiko),
                        2 => Some(GameMode::Catch),
                        3 => Some(GameMode::Mania),
                        _ => return Err(PyValueError::new_err("invalid mode integer")),
                    }
                }
                "shaymi_mode" => {
                    this.shaymi_mode = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'shaymi_mode': must be a boolean"))?;
                }
                "mods" => {
                    this.mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'mods': must be an int"))?;
                }
                "n300" => {
                    this.n300 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n300': must be an int"))?;
                }
                "n100" => {
                    this.n100 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n100': must be an int"))?;
                }
                "n50" => {
                    this.n50 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n50': must be an int"))?;
                }
                "n_misses" => {
                    this.n_misses = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_misses': must be an int"))?;
                }
                "n_geki" => {
                    this.n_geki = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_geki': must be an int"))?;
                }
                "n_katu" => {
                    this.n_katu = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_katu': must be an int"))?;
                }
                "acc" | "accuracy" => {
                    this.acc = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'acc': must be a real number"))?;
                }
                "combo" => {
                    this.combo = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'combo': must be an int"))?;
                }
                "ac" => {
                    this.ac = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'ac': must be an int"))?;
                }
                "arc" => {
                    this.arc = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'arc': must be a real number"))?;
                }
                "hdr" => {
                    this.hdr = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'hdr': must be a boolean"))?;
                }
                "tw" => {
                    this.hdr = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'tw': must be an int"))?;
                }
                "cs" => {
                    this.hdr = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'cs': must be a boolean"))?;
                }
                "passed_objects" => {
                    this.passed_objects = value.extract().map_err(|_| {
                        PyTypeError::new_err("kwarg 'passed_objects': must be an int")
                    })?;
                }
                "clock_rate" => {
                    this.clock_rate = value.extract().map_err(|_| {
                        PyTypeError::new_err("kwarg 'clock_rate': must be a real number")
                    })?;
                }
                "difficulty" | "attributes" => {
                    let attrs = value.extract::<PyDifficultyAttributes>().map_err(|_| {
                        PyTypeError::new_err("kwarg 'difficulty': must be DifficultyAttributes")
                    })?;

                    this.attributes = Some(attrs.inner);
                }
                "notrefx" => {
                    this.notrefx = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'notrefx': must be a boolean"))?;
                }
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'mode', 'mods', \n\
                        'n_geki', 'n_katu', 'n300', 'n100', 'n50', 'n_misses', \n\
                        'acc', 'combo', 'passed_objects', 'clock_rate', or 'difficulty'"
                    );

                    return Err(KwargsError::new_err(err));
                }
            }
        }

        Ok(this)
    }

    fn set_mods(&mut self, mods: u32) {
        self.mods = Some(mods);
    }

    fn set_acc(&mut self, acc: f64) {
        self.acc = Some(acc);
    }

    fn set_n_geki(&mut self, n_geki: usize) {
        self.n_geki = Some(n_geki);
    }

    fn set_n_katu(&mut self, n_katu: usize) {
        self.n_katu = Some(n_katu);
    }

    fn set_n300(&mut self, n300: usize) {
        self.n300 = Some(n300);
    }

    fn set_n100(&mut self, n100: usize) {
        self.n100 = Some(n100);
    }

    fn set_n50(&mut self, n50: usize) {
        self.n50 = Some(n50);
    }

    fn set_n_misses(&mut self, n_misses: usize) {
        self.n_misses = Some(n_misses);
    }

    fn set_combo(&mut self, combo: usize) {
        self.combo = Some(combo);
    }

    fn cheat_ac(&mut self, ac: usize) {
        self.ac = Some(ac);
    }

    fn cheat_arc(&mut self, arc: f64) {
        self.arc = Some(arc);
    }

    fn cheat_hdr(&mut self, hdr: bool) {
        self.hdr = Some(hdr);
    }

    fn cheat_tw(&mut self, tw: usize) {
        self.tw = Some(tw);
    }

    fn cheat_cs(&mut self, cs: bool) {
        self.cs = Some(cs);
    }

    fn set_passed_objects(&mut self, passed_objects: usize) {
        self.passed_objects = Some(passed_objects);
    }

    fn set_clock_rate(&mut self, clock_rate: f64) {
        self.clock_rate = Some(clock_rate);
    }

    fn set_difficulty(&mut self, difficulty: PyDifficultyAttributes) {
        self.attributes = Some(difficulty.inner);
    }

    fn map_attributes(&self, map: &PyBeatmap) -> PyResult<PyBeatmapAttributes> {
        let map = &map.inner;
        let mut calc = map.attributes();

        if let Some(mode) = self.mode {
            calc.mode(mode);

            if map.mode != mode && map.mode == GameMode::Osu {
                calc.converted(true);
            }
        }

        if let Some(mods) = self.mods {
            calc.mods(mods);
        }

        if let Some(clock_rate) = self.clock_rate {
            calc.clock_rate(clock_rate);
        }

        Ok(PyBeatmapAttributes::new(calc.build(), map))
    }

    fn difficulty(&self, map: &PyBeatmap) -> PyResult<PyDifficultyAttributes> {
        let mut calc = AnyStars::new(&map.inner);

        set_calc! { calc, self:
            mode,
            mods,
            passed_objects,
            clock_rate,
        };

        Ok(calc.calculate().into())
    }

    fn performance_2019(&self, map: &PyBeatmap) -> PyResult<PyPerformanceAttributes> {
        let mut calc = OsuPP::new(&map.inner);

        set_calc! { calc, self:
            mods,
            combo,
            n300,
            n100,
            n50,
            passed_objects,
            ac,
            arc,
            hdr,
            tw,
            cs,
        };

        if let Some(n_misses) = self.n_misses {
            calc = calc.misses(n_misses);
        }

        if let Some(acc) = self.acc {
            calc = calc.accuracy(acc as f32);
        }

        let attrs = calc.calculate();

        let new_attrs = OsuPerformanceAttributes {
            difficulty: OsuDifficultyAttributes {
                aim: attrs.difficulty.aim_strain,
                speed: attrs.difficulty.speed_strain,
                flashlight: 0.0,
                slider_factor: 0.0,
                speed_note_count: 0.0,
                ar: attrs.difficulty.ar,
                od: attrs.difficulty.od,
                hp: attrs.difficulty.hp,
                cs: attrs.difficulty.cs,
                n_circles: attrs.difficulty.n_circles,
                n_sliders: attrs.difficulty.n_sliders,
                n_spinners: attrs.difficulty.n_spinners,
                stars: attrs.difficulty.stars,
                max_combo: attrs.difficulty.max_combo,
                aim_difficult_strain_count: 0.0,
                speed_difficult_strain_count: 0.0,
            },
            pp: attrs.pp,
            pp_acc: attrs.pp_acc,
            pp_aim: attrs.pp_aim,
            pp_flashlight: attrs.pp_flashlight,
            pp_speed: attrs.pp_speed,
            effective_miss_count: attrs.effective_miss_count,
        };

        Ok(PerformanceAttributes::Osu(new_attrs).into())
    }
    
    fn performance_notrefx(&self, map: &PyBeatmap) -> PyResult<PyPerformanceAttributes> {
        let mut calc = FxPP::new_from_map(&map.inner);

        set_calc! { calc, self:
            mods,
            combo,
            n300,
            n100,
            n50,
            passed_objects,
        };

        if let Some(n_misses) = self.n_misses {
            calc = calc.misses(n_misses);
        }

        if let Some(acc) = self.acc {
            calc = calc.accuracy(acc as f32);
        }

        let attrs = calc.calculate();

        let new_attrs = OsuPerformanceAttributes {
            difficulty: OsuDifficultyAttributes {
                aim: attrs.difficulty.aim_strain,
                speed: attrs.difficulty.speed_strain,
                flashlight: 0.0,
                slider_factor: 0.0,
                speed_note_count: 0.0,
                ar: attrs.difficulty.ar,
                od: attrs.difficulty.od,
                hp: attrs.difficulty.hp,
                cs: attrs.difficulty.cs,
                n_circles: attrs.difficulty.n_circles,
                n_sliders: attrs.difficulty.n_sliders,
                n_spinners: attrs.difficulty.n_spinners,
                stars: attrs.difficulty.stars,
                max_combo: attrs.difficulty.max_combo,
                aim_difficult_strain_count: 0.0,
                speed_difficult_strain_count: 0.0,
            },
            pp: attrs.pp,
            pp_acc: attrs.pp_acc,
            pp_aim: attrs.pp_aim,
            pp_flashlight: attrs.pp_flashlight,
            pp_speed: attrs.pp_speed,
            effective_miss_count: attrs.effective_miss_count,
        };

        Ok(PerformanceAttributes::Osu(new_attrs).into())
    }

    fn performance(&self, map: &PyBeatmap) -> PyResult<PyPerformanceAttributes> {
        // * if a player isnt playing on refx client, we check for ScoreV2 mod and we return
        // * the other client calculation https://github.com/refx-online/refx-pp-rs/pull/1
        // * and is osu!standard. or mode is not specified and map is osu!standard, as that will be the inferred mode
        if (self.mods.is_some() && self.mods.unwrap().sv2()) || self.notrefx
            && ((self.mode.is_none() && map.inner.mode == GameMode::Osu)
                || self.mode == Some(GameMode::Osu)) {
            return self.performance_notrefx(map);
        }
        // criteria:
        // - is relax
        // - is osu!standard
        // - is shaymi
        //   or mode is not specified and map is osu!standard, as that will be the inferred mode
        if (self.mods.is_some() && self.mods.unwrap().rx()) || self.shaymi_mode
            && ((self.mode.is_none() && map.inner.mode == GameMode::Osu)
                || self.mode == Some(GameMode::Osu))
        {
            return self.performance_2019(map);
        }

        let mut calc = AnyPP::new(&map.inner);

        set_calc! { calc, self:
            mode,
            mods,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            n_misses,
            combo,
            ac,
            arc,
            hdr,
            passed_objects,
            clock_rate,
        };

        if let Some(ref attrs) = self.attributes {
            calc = calc.attributes(attrs.to_owned());
        }

        if let Some(acc) = self.acc {
            calc = calc.accuracy(acc);
        }

        Ok(calc.calculate().into())
    }

    fn strains(&self, map: &PyBeatmap) -> PyResult<PyStrains> {
        let mut calc = AnyStars::new(&map.inner);

        set_calc! { calc, self:
            mode,
            mods,
            passed_objects,
            clock_rate,
        };

        Ok(calc.strains().into())
    }
}
