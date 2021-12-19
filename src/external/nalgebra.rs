use hv_alchemy::Type;
use hv_math::Normed;
use nalgebra::{Isometry2, Isometry3, Point2, Point3, RealField, Unit, Vector2, Vector3};
use std::fmt;

use crate::{
    AnyUserData, FromLua, Lua, MetaMethod, Result, Table, ToLua, UserData, UserDataFields,
    UserDataMethods, Value,
};

pub trait LuaRealField: RealField + Copy + for<'lua> ToLua<'lua> + for<'lua> FromLua<'lua> {}
impl<T> LuaRealField for T where
    T: RealField + Copy + for<'lua> ToLua<'lua> + for<'lua> FromLua<'lua> + Send + Sync
{
}

macro_rules! get_set_coords {
    ($fields:ident, $($a:tt),*) => {{$(
        $fields.add_field_method_get(stringify!($a), |_, this| Ok(this.$a));
        #[allow(clippy::unit_arg)]
        $fields.add_field_method_set(stringify!($a), |_, this, a| Ok(this.$a = a));
    )*}}
}

impl<T: LuaRealField> UserData for Vector2<T> {
    fn on_metatable_init(table: Type<Self>) {
        table
            .add_clone()
            .add_copy()
            .add_send()
            .add_sync()
            .add::<dyn fmt::Debug>();
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        get_set_coords!(fields, x, y);
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set", |_, this, (x, y): (T, T)| {
            *this = Self::new(x, y);
            Ok(())
        });

        methods.add_function(
            "add",
            |lua, (a, b, out): (Self, Self, Option<AnyUserData>)| match out {
                Some(ud) => {
                    *ud.borrow_mut::<Self>()? = a + b;
                    Ok(ud)
                }
                None => lua.create_userdata(a + b),
            },
        );

        methods.add_function(
            "sub",
            |lua, (a, b, out): (Self, Self, Option<AnyUserData>)| match out {
                Some(ud) => {
                    *ud.borrow_mut::<Self>()? = a - b;
                    Ok(ud)
                }
                None => lua.create_userdata(a - b),
            },
        );

        methods.add_meta_function(MetaMethod::Add, |_, (a, b): (Self, Self)| Ok(a + b));
        methods.add_meta_function(MetaMethod::Sub, |_, (a, b): (Self, Self)| Ok(a - b));
        methods.add_meta_function(MetaMethod::Unm, |_, a: Self| Ok(-a));

        methods.add_method_mut("normalize_mut", |_, this, ()| {
            this.normalize_mut();
            Ok(())
        });

        methods.add_method("norm", |_, this, ()| Ok(this.norm()));
    }

    fn add_type_methods<'lua, M: UserDataMethods<'lua, Type<Self>>>(methods: &mut M)
    where
        Self: 'static,
    {
        methods.add_function("new", |_, (x, y): (T, T)| Ok(Self::new(x, y)));
        methods.add_function("zeros", |_, ()| Ok(Self::zeros()));
        methods.add_function("x_axis", |_, ()| Ok(Self::x()));
        methods.add_function("y_axis", |_, ()| Ok(Self::y()));
    }
}

impl<T: LuaRealField> UserData for Vector3<T> {
    fn on_metatable_init(table: Type<Self>) {
        table
            .add_clone()
            .add_copy()
            .add_send()
            .add_sync()
            .add::<dyn fmt::Debug>()
            .add_conversion_from::<Vector3<T>>();
    }

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        get_set_coords!(fields, x, y, z);
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set", |_, this, (x, y, z): (T, T, T)| {
            *this = Self::new(x, y, z);
            Ok(())
        });

        methods.add_function(
            "add",
            |lua, (out, a, b): (Option<AnyUserData>, Self, Self)| match out {
                Some(ud) => {
                    *ud.borrow_mut::<Self>()? = a + b;
                    Ok(ud)
                }
                None => lua.create_userdata(a + b),
            },
        );

        methods.add_function(
            "sub",
            |lua, (out, a, b): (Option<AnyUserData>, Self, Self)| match out {
                Some(ud) => {
                    *ud.borrow_mut::<Self>()? = a - b;
                    Ok(ud)
                }
                None => lua.create_userdata(a - b),
            },
        );

        methods.add_meta_function(MetaMethod::Add, |_, (a, b): (Self, Self)| Ok(a + b));
        methods.add_meta_function(MetaMethod::Sub, |_, (a, b): (Self, Self)| Ok(a - b));
        methods.add_meta_function(MetaMethod::Unm, |_, a: Self| Ok(-a));

        methods.add_method_mut("normalize_mut", |_, this, ()| {
            this.normalize_mut();
            Ok(())
        });

        methods.add_method("norm", |_, this, ()| Ok(this.norm()));
    }

    fn add_type_methods<'lua, M: UserDataMethods<'lua, Type<Self>>>(methods: &mut M)
    where
        Self: 'static,
    {
        methods.add_function("new", |_, (x, y, z): (T, T, T)| Ok(Self::new(x, y, z)));
        methods.add_function("zeros", |_, ()| Ok(Self::zeros()));
        methods.add_function("x_axis", |_, ()| Ok(Self::x()));
        methods.add_function("y_axis", |_, ()| Ok(Self::y()));
        methods.add_function("z_axis", |_, ()| Ok(Self::z()));
    }
}

impl<'lua, T: LuaRealField> FromLua<'lua> for Point2<T> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        Vector2::from_lua(lua_value, lua).map(Point2::from)
    }
}

impl<'lua, T: LuaRealField> ToLua<'lua> for Point2<T> {
    fn to_lua(self, lua: &'lua Lua) -> Result<Value<'lua>> {
        Vector2::to_lua(self.coords, lua)
    }
}

impl<'lua, T: LuaRealField> FromLua<'lua> for Point3<T> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        Vector3::from_lua(lua_value, lua).map(Point3::from)
    }
}

impl<'lua, T: LuaRealField> ToLua<'lua> for Point3<T> {
    fn to_lua(self, lua: &'lua Lua) -> Result<Value<'lua>> {
        Vector3::to_lua(self.coords, lua)
    }
}

impl<T: LuaRealField> UserData for Isometry2<T> {
    fn on_metatable_init(table: Type<Self>) {
        table
            .add_clone()
            .add_copy()
            .add_send()
            .add_sync()
            .add::<dyn fmt::Debug>();
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {}

    fn add_type_methods<'lua, M: UserDataMethods<'lua, Type<Self>>>(methods: &mut M)
    where
        Self: 'static,
    {
        methods.add_function("new", |_, (t, a): (Vector2<T>, T)| Ok(Self::new(t, a)));
        methods.add_function("translation", |_, (x, y): (T, T)| {
            Ok(Self::translation(x, y))
        });
        methods.add_function("rotation", |_, angle: T| Ok(Self::rotation(angle)));
        methods.add_function("identity", |_, ()| Ok(Self::identity()));
    }
}

impl<T: LuaRealField> UserData for Isometry3<T> {
    fn on_metatable_init(table: Type<Self>) {
        table
            .add_clone()
            .add_copy()
            .add_send()
            .add_sync()
            .add::<dyn fmt::Debug>();
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (out, a, b): (Option<AnyUserData>, Self, Self)| match out {
                Some(ud) => {
                    *ud.borrow_mut::<Self>()? = a * b;
                    Ok(ud)
                }
                None => lua.create_userdata(a * b),
            },
        );

        methods.add_function(
            "div",
            |lua, (out, a, b): (Option<AnyUserData>, Self, Self)| match out {
                Some(ud) => {
                    *ud.borrow_mut::<Self>()? = a / b;
                    Ok(ud)
                }
                None => lua.create_userdata(a / b),
            },
        );
    }

    fn add_type_methods<'lua, M: UserDataMethods<'lua, Type<Self>>>(methods: &mut M)
    where
        Self: 'static,
    {
        methods.add_function("new", |_, (t, a): (Vector3<T>, Vector3<T>)| {
            Ok(Self::new(t, a))
        });
        methods.add_function("translation", |_, (x, y, z): (T, T, T)| {
            Ok(Self::translation(x, y, z))
        });
        methods.add_function("rotation", |_, axis_angle: Vector3<T>| {
            Ok(Self::rotation(axis_angle))
        });
        methods.add_function("identity", |_, ()| Ok(Self::identity()));
    }
}

impl<'lua, T: ToLua<'lua>> ToLua<'lua> for Unit<T> {
    fn to_lua(self, lua: &'lua Lua) -> Result<Value<'lua>> {
        self.into_inner().to_lua(lua)
    }
}

impl<'lua, T: FromLua<'lua>> FromLua<'lua> for Unit<T>
where
    T: Normed,
{
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        Ok(Unit::new_normalize(T::from_lua(lua_value, lua)?))
    }
}

pub struct Module;

impl<'lua> ToLua<'lua> for Module {
    fn to_lua(self, lua: &'lua Lua) -> Result<Value<'lua>> {
        table(lua).map(Value::Table)
    }
}

pub fn table(lua: &Lua) -> Result<Table> {
    let src = "return family[...]";

    macro_rules! e {
            ($lua:ident, $name:ident($($ty:ty),*)) => {{
                let t = $lua.create_table()?;
                $(t.set(stringify!($ty), lua.create_userdata_type::<$name<$ty>>()?)?;)*
                let env = lua.create_table_from(vec![("family", t)])?;
                let f = lua.load(src).set_environment(env)?.into_function()?;
                (stringify!($name), f)
            }};
        }

    macro_rules! types {
            ($lua:ident, $($name:ident($($field:ty),*)),* $(,)?) => { vec![$(e!($lua, $name($($field),*))),*] };
        }

    let es = types! {lua,
        Vector2(f32, f64),
        Vector3(f32, f64),

        Isometry2(f32, f64),
        Isometry3(f32, f64),
    };

    lua.create_table_from(es)
}
