#![allow(warnings)]

use expect_test::expect;
use salsa::DebugWithDb;

#[salsa::jar(db = Db)]
struct Jar(MyInput);

trait Db: salsa::DbWithJar<Jar> {}

// Recursive expansion of input! macro
// ====================================

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
struct MyInput(salsa::Id);

impl MyInput {
    pub fn new(
        __db: &<Jar as salsa::jar::Jar<'_>>::DynDb,
        field: u32,
        id_one: u32,
        id_two: u16,
    ) -> Self {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<MyInput>>::ingredient(__jar);
        let __id = __ingredients.3.new_input(__runtime);
        __ingredients
            .0
            .store_new(__runtime, __id, field, salsa::Durability::LOW);
        __ingredients
            .1
            .store_new(__runtime, __id, id_one, salsa::Durability::LOW);
        __ingredients
            .2
            .store_new(__runtime, __id, id_two, salsa::Durability::LOW);
        __id
    }
    fn field<'db>(self, __db: &'db <Jar as salsa::jar::Jar<'_>>::DynDb) -> u32 {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<MyInput>>::ingredient(__jar);
        __ingredients.0.fetch(__runtime, self).clone()
    }
    fn id_one<'db>(self, __db: &'db <Jar as salsa::jar::Jar<'_>>::DynDb) -> u32 {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<MyInput>>::ingredient(__jar);
        __ingredients.1.fetch(__runtime, self).clone()
    }
    fn id_two<'db>(self, __db: &'db <Jar as salsa::jar::Jar<'_>>::DynDb) -> u16 {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar(__db);
        let __ingredients = <Jar as salsa::storage::HasIngredientsFor<MyInput>>::ingredient(__jar);
        __ingredients.2.fetch(__runtime, self).clone()
    }
    fn set_field<'db>(
        self,
        __db: &'db mut <Jar as salsa::jar::Jar<'_>>::DynDb,
    ) -> salsa::setter::Setter<'db, MyInput, u32> {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<Jar>>::jar_mut(__db);
        let __ingredients =
            <Jar as salsa::storage::HasIngredientsFor<MyInput>>::ingredient_mut(__jar);
        salsa::setter::Setter::new(__runtime, self, &mut __ingredients.0)
    }
}
impl salsa::storage::IngredientsFor for MyInput {
    type Jar = Jar;
    type Ingredients = (
        salsa::input_field::InputFieldIngredient<MyInput, u32>,
        salsa::input_field::InputFieldIngredient<MyInput, u32>,
        salsa::input_field::InputFieldIngredient<MyInput, u16>,
        salsa::input::InputIngredient<MyInput>,
    );
    fn create_ingredients<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        (
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.0
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.0
                    },
                );
                salsa::input_field::InputFieldIngredient::new(index, "field")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.1
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.1
                    },
                );
                salsa::input_field::InputFieldIngredient::new(index, "id_one")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.2
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.2
                    },
                );
                salsa::input_field::InputFieldIngredient::new(index, "id_two")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.3
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.3
                    },
                );
                salsa::input::InputIngredient::new(index, "MyInput")
            },
        )
    }
}
impl salsa::AsId for MyInput {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        MyInput(id)
    }
}
impl<_Db: Db + ?Sized> ::salsa::DebugWithDb<_Db> for MyInput {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &_Db,
        _level: salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let _db = <_Db as ::salsa::DbWithJar<Jar>>::as_jar_db(_db);
        let mut debug_struct = &mut f.debug_struct("MyInput");
        if _level.is_root() {
            debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        }
        debug_struct = debug_struct.field("field", & ::salsa::debug::helper::SalsaDebug:: <u32, <Jar as salsa::jar::Jar<'_> > ::DynDb> ::salsa_debug(#[allow(clippy::needless_borrow)]
    &self.field(_db),_db,_level.next()));
        debug_struct = debug_struct.field("id_one", & ::salsa::debug::helper::SalsaDebug:: <u32, <Jar as salsa::jar::Jar<'_> > ::DynDb> ::salsa_debug(#[allow(clippy::needless_borrow)]
    &self.id_one(_db),_db,_level.next()));
        debug_struct = debug_struct.field("id_two", & ::salsa::debug::helper::SalsaDebug:: <u16, <Jar as salsa::jar::Jar<'_> > ::DynDb> ::salsa_debug(#[allow(clippy::needless_borrow)]
    &self.id_two(_db),_db,_level.next()));
        debug_struct.finish()
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for MyInput
where
    DB: ?Sized + salsa::DbWithJar<Jar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}

impl Db for Database {}

#[test]
fn test_debug() {
    let mut db = Database::default();

    let input = MyInput::new(&mut db, 22, 50, 10);

    let actual = format!("{:?}", input.debug(&db));
    let expected = expect!["MyInput { field: 22, id_one: 50, id_two: 10 }"];
    expected.assert_eq(&actual);
}
