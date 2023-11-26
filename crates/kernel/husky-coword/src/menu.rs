use salsa::Db;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CowordMenu {
    std_name: Kebab,
    std_ident: Ident,
    core_name: Kebab,
    core_ident: Ident,
    unit_ident: Ident,
    never_ident: Ident,
    bool_ident: Ident,
    derive_ident: Ident,
    i8_ident: Ident,
    i16_ident: Ident,
    i32_ident: Ident,
    i64_ident: Ident,
    i128_ident: Ident,
    isize_ident: Ident,
    u8_ident: Ident,
    u16_ident: Ident,
    u32_ident: Ident,
    u64_ident: Ident,
    u128_ident: Ident,
    usize_ident: Ident,
    r8_ident: Ident,
    r16_ident: Ident,
    r32_ident: Ident,
    r64_ident: Ident,
    r128_ident: Ident,
    rsize_ident: Ident,
    f32_ident: Ident,
    f64_ident: Ident,
    trai_ty_ident: Ident,
    lifetime_ty_ident: Ident,
    place_ty_ident: Ident,
    module_ident: Ident,
    crate_ident: Ident,
    camel_case_output_ident: Ident,
}

impl CowordMenu {
    pub(crate) fn new(db: &Db) -> Self {
        Self {
            core_name: Kebab::from_ref(db, "core").unwrap(),
            core_ident: Ident::from_borrowed(db, "core").unwrap(),
            std_name: Kebab::from_ref(db, "std").unwrap(),
            std_ident: Ident::from_borrowed(db, "std").unwrap(),
            unit_ident: db.it_ident_borrowed("unit").unwrap(),
            never_ident: db.it_ident_borrowed("never").unwrap(),
            bool_ident: db.it_ident_borrowed("bool").unwrap(),
            derive_ident: db.it_ident_borrowed("derive").unwrap(),
            i8_ident: db.it_ident_borrowed("i8").unwrap(),
            i16_ident: db.it_ident_borrowed("i16").unwrap(),
            i32_ident: db.it_ident_borrowed("i32").unwrap(),
            i64_ident: db.it_ident_borrowed("i64").unwrap(),
            i128_ident: db.it_ident_borrowed("i128").unwrap(),
            isize_ident: db.it_ident_borrowed("isize").unwrap(),
            u8_ident: db.it_ident_borrowed("u8").unwrap(),
            u16_ident: db.it_ident_borrowed("u16").unwrap(),
            u32_ident: db.it_ident_borrowed("u32").unwrap(),
            u64_ident: db.it_ident_borrowed("u64").unwrap(),
            u128_ident: db.it_ident_borrowed("u128").unwrap(),
            usize_ident: db.it_ident_borrowed("usize").unwrap(),
            r8_ident: db.it_ident_borrowed("r8").unwrap(),
            r16_ident: db.it_ident_borrowed("r16").unwrap(),
            r32_ident: db.it_ident_borrowed("r32").unwrap(),
            r64_ident: db.it_ident_borrowed("r64").unwrap(),
            r128_ident: db.it_ident_borrowed("r128").unwrap(),
            rsize_ident: db.it_ident_borrowed("rsize").unwrap(),
            f32_ident: db.it_ident_borrowed("f32").unwrap(),
            f64_ident: db.it_ident_borrowed("f64").unwrap(),
            trai_ty_ident: db.it_ident_borrowed("Trait").unwrap(),
            module_ident: db.it_ident_borrowed("Module").unwrap(),
            crate_ident: db.it_ident_borrowed("crate").unwrap(),
            lifetime_ty_ident: db.it_ident_borrowed("Lifetime").unwrap(),
            place_ty_ident: db.it_ident_borrowed("Place").unwrap(),
            camel_case_output_ident: db.it_ident_borrowed("Output").unwrap(),
        }
    }

    pub fn core_name(&self) -> Kebab {
        self.core_name
    }

    pub fn core_ident(&self) -> Ident {
        self.core_ident
    }

    pub fn std_name(&self) -> Kebab {
        self.std_name
    }

    pub fn std_ident(&self) -> Ident {
        self.std_ident
    }

    pub fn i8_ident(&self) -> Ident {
        self.i8_ident
    }

    pub fn i16_ident(&self) -> Ident {
        self.i16_ident
    }

    pub fn i32_ident(&self) -> Ident {
        self.i32_ident
    }

    pub fn i64_ident(&self) -> Ident {
        self.i64_ident
    }

    pub fn i128_ident(&self) -> Ident {
        self.i128_ident
    }

    pub fn isize_ident(&self) -> Ident {
        self.isize_ident
    }

    pub fn r8_ident(&self) -> Ident {
        self.r8_ident
    }

    pub fn r16_ident(&self) -> Ident {
        self.r16_ident
    }

    pub fn r32_ident(&self) -> Ident {
        self.r32_ident
    }

    pub fn r64_ident(&self) -> Ident {
        self.r64_ident
    }

    pub fn r128_ident(&self) -> Ident {
        self.r128_ident
    }

    pub fn rsize_ident(&self) -> Ident {
        self.rsize_ident
    }

    pub fn f32_ident(&self) -> Ident {
        self.f32_ident
    }

    pub fn f64_ident(&self) -> Ident {
        self.f64_ident
    }

    pub fn unit_ident(&self) -> Ident {
        self.unit_ident
    }

    pub fn bool_ident(&self) -> Ident {
        self.bool_ident
    }

    pub fn u8_ident(&self) -> Ident {
        self.u8_ident
    }

    pub fn u16_ident(&self) -> Ident {
        self.u16_ident
    }

    pub fn u32_ident(&self) -> Ident {
        self.u32_ident
    }

    pub fn u64_ident(&self) -> Ident {
        self.u64_ident
    }

    pub fn u128_ident(&self) -> Ident {
        self.u128_ident
    }

    pub fn usize_ident(&self) -> Ident {
        self.usize_ident
    }

    pub fn trai_ty_ident(&self) -> Ident {
        self.trai_ty_ident
    }

    pub fn module_ident(&self) -> Ident {
        self.module_ident
    }

    pub fn crate_ident(&self) -> Ident {
        self.crate_ident
    }

    pub fn lifetime_ty_ident(&self) -> Ident {
        self.lifetime_ty_ident
    }

    pub fn place_ty_ident(&self) -> Ident {
        self.place_ty_ident
    }

    pub fn never_ident(&self) -> Ident {
        self.never_ident
    }

    pub fn derive_ident(&self) -> Ident {
        self.derive_ident
    }

    pub fn camel_case_output_ident(&self) -> Ident {
        self.camel_case_output_ident
    }
}

// #[salsa::tracked(jar = CowordJar, return_ref)]
// pub(crate) fn ident_menu(db: &Db) -> CowordMenu {
//     CowordMenu::new(db)
// }

#[allow(non_camel_case_types)]
pub(crate) struct ident_menu {
    intern_map: salsa::interned::IdentityInterner<()>,
    function: salsa::function::FunctionIngredient<Self>,
}
impl salsa::function::Configuration for ident_menu {
    type Jar = CowordJar;
    type SalsaStruct = salsa::salsa_struct::Singleton;
    type Key = ();
    type Value = CowordMenu;
    const CYCLE_STRATEGY: salsa::cycle::CycleRecoveryStrategy =
        salsa::cycle::CycleRecoveryStrategy::Panic;
    fn should_backdate_value(v1: &Self::Value, v2: &Self::Value) -> bool {
        salsa::function::should_backdate_value(v1, v2)
    }
    fn execute(__db: &::salsa::Db, __id: Self::Key) -> Self::Value {
        pub(crate) fn __fn(db: &Db) -> CowordMenu {
            CowordMenu::new(db)
        }
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(__db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<ident_menu>>::ingredient(__jar);
        let __key = __ingredients.intern_map.data(__runtime, __id).clone();
        __fn(__db)
    }
    fn recover_from_cycle(_db: &Db, _cycle: &salsa::Cycle, _key: Self::Key) -> Self::Value {
        panic!()
    }
}
impl salsa::storage::IngredientsFor for ident_menu {
    type Ingredients = Self;
    type Jar = CowordJar;
    fn create_ingredients(routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        Self {
            intern_map: salsa::interned::IdentityInterner::new(),
            function: {
                let index = routes.push(
                    |jars| {
                        let jar = jars.jar::<Self::Jar>();
                        let ingredients = <_ as salsa::storage::HasIngredientsFor<
                            Self::Ingredients,
                        >>::ingredient(jar);
                        &ingredients.function
                    },
                    |jars| {
                        let jar = jars.jar_mut::<Self::Jar>();
                        let ingredients = <_ as salsa::storage::HasIngredientsFor<
                            Self::Ingredients,
                        >>::ingredient_mut(jar);
                        &mut ingredients.function
                    },
                );
                let ingredient = salsa::function::FunctionIngredient::new(index, "ident_menu");
                ingredient.set_capacity(0usize);
                ingredient
            },
        }
    }
}
impl ident_menu {
    #[allow(dead_code, clippy::needless_lifetimes)]
    pub(crate) fn get<'__db>(db: &'__db Db) -> &'__db CowordMenu {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<ident_menu>>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, ());
        __ingredients.function.fetch(db, __key)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    pub(crate) fn set(db: &mut Db, __value: CowordMenu) {
        let (__jar, __runtime) = db.jar_mut();
        let __ingredients =
            <_ as salsa::storage::HasIngredientsFor<ident_menu>>::ingredient_mut(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, ());
        __ingredients
            .function
            .store(__runtime, __key, __value, salsa::Durability::LOW)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    pub(crate) fn accumulated<'__db, __A: salsa::accumulator::Accumulator>(
        db: &'__db Db,
    ) -> Vec<<__A as salsa::accumulator::Accumulator>::Data> {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(db);
        let __ingredients = <_ as salsa::storage::HasIngredientsFor<ident_menu>>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, ());
        __ingredients.function.accumulated::<__A>(db, __key)
    }
}
#[allow(clippy::needless_lifetimes)]
pub(crate) fn ident_menu<'__db>(db: &'__db Db) -> &'__db CowordMenu {
    ident_menu::get(db)
}
