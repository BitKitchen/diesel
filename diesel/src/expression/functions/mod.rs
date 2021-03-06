#[macro_export]
#[doc(hidden)]
#[cfg(feature = "with-deprecated")]
#[deprecated(since = "1.3.0", note = "The syntax of `sql_function!` and its output have changed significantly. This form has been deprecated. See the documentation of `sql_function!` for details on the new syntax.")]
macro_rules! sql_function_body {
    (
        $fn_name:ident,
        $struct_name:ident,
        ($($arg_name:ident: $arg_type:ty),*) -> $return_type:ty,
        $docs:expr,
        $helper_ty_docs:expr
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, QueryId)]
        #[doc(hidden)]
        pub struct $struct_name<$($arg_name),*> {
            $($arg_name: $arg_name),*
        }

        #[allow(non_camel_case_types)]
        #[doc=$helper_ty_docs]
        pub type $fn_name<$($arg_name),*> = $struct_name<$(
            <$arg_name as $crate::expression::AsExpression<$arg_type>>::Expression
        ),*>;

        #[allow(non_camel_case_types)]
        #[doc=$docs]
        pub fn $fn_name<$($arg_name),*>($($arg_name: $arg_name),*)
            -> $fn_name<$($arg_name),*>
            where $($arg_name: $crate::expression::AsExpression<$arg_type>),+
        {
            $struct_name {
                $($arg_name: $arg_name.as_expression()),+
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($arg_name),*> $crate::expression::Expression for $struct_name<$($arg_name),*> where
            for <'a> ($(&'a $arg_name),*): $crate::expression::Expression,
        {
            type SqlType = $return_type;
        }

        #[allow(non_camel_case_types)]
        impl<$($arg_name),*, DB> $crate::query_builder::QueryFragment<DB> for $struct_name<$($arg_name),*> where
            DB: $crate::backend::Backend,
            for <'a> ($(&'a $arg_name),*): $crate::query_builder::QueryFragment<DB>,
        {
            fn walk_ast(&self, mut out: $crate::query_builder::AstPass<DB>) -> $crate::result::QueryResult<()> {
                out.push_sql(concat!(stringify!($fn_name), "("));
                $crate::query_builder::QueryFragment::walk_ast(
                    &($(&self.$arg_name),*), out.reborrow())?;
                out.push_sql(")");
                Ok(())
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($arg_name),*, QS> $crate::expression::SelectableExpression<QS> for $struct_name<$($arg_name),*> where
            $($arg_name: $crate::expression::SelectableExpression<QS>,)*
            $struct_name<$($arg_name),*>: $crate::expression::AppearsOnTable<QS>,
        {
        }

        #[allow(non_camel_case_types)]
        impl<$($arg_name),*, QS> $crate::expression::AppearsOnTable<QS> for $struct_name<$($arg_name),*> where
            $($arg_name: $crate::expression::AppearsOnTable<QS>,)*
            $struct_name<$($arg_name),*>: $crate::expression::Expression,
        {
        }

        #[allow(non_camel_case_types)]
        impl<$($arg_name),*> $crate::expression::NonAggregate for $struct_name<$($arg_name),*> where
            $($arg_name: $crate::expression::NonAggregate,)*
            $struct_name<$($arg_name),*>: $crate::expression::Expression,
        {
        }
    }
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "with-deprecated"))]
macro_rules! sql_function_body {
    ($($args:tt)*) => {
        compile_error!("You are using a deprecated form of `sql_function!`. \
        You must enable the `with-deprecated` feature on `diesel`.");
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __diesel_sql_function_body {
    // Entry point. We need to search the meta items for our special attributes
    (
        meta = $meta:tt,
        $($rest:tt)*
    ) => {
        __diesel_sql_function_body! {
            unchecked_meta = $meta,
            meta = (),
            $($rest)*
        }
    };

    // Searching for `#[aggregate]`, found it.
    (
        unchecked_meta = (#[aggregate] $($unchecked:tt)*),
        meta = ($($meta:tt)*),
        $($rest:tt)*
    ) => {
        __diesel_sql_function_body! {
            aggregate = yes,
            meta = ($($meta)* $($unchecked)*),
            $($rest)*
        }
    };

    // Searching for `#[aggregate]`. Didn't find it.
    (
        unchecked_meta = (#$checked:tt $($unchecked:tt)*),
        meta = ($($meta:tt)*),
        $($rest:tt)*
    ) => {
        __diesel_sql_function_body! {
            unchecked_meta = ($($unchecked)*),
            meta = ($($meta)* #$checked),
            $($rest)*
        }
    };

    // Done searching for `#[aggregate]`.
    (
        unchecked_meta = (),
        meta = $meta:tt,
        $($rest:tt)*
    ) => {
        __diesel_sql_function_body! {
            aggregate = no,
            meta = $meta,
            $($rest)*
        }
    };

    (
        aggregate = $aggregate:tt,
        meta = ($($meta:tt)*),
        fn_name = $fn_name:ident,
        type_args = ($($type_args:ident,)*),
        type_args_with_bounds = ($($type_args_bounds:tt)*),
        args = ($($arg_name:ident: $arg_type:ty),*),
        return_type = $return_type:ty,
    ) => {
        $($meta)*
        #[allow(non_camel_case_types)]
        pub fn $fn_name<$($type_args_bounds)* $($arg_name),*>($($arg_name: $arg_name),*)
            -> $fn_name::HelperType<$($type_args,)* $($arg_name),*>
        where
            $($arg_name: $crate::expression::AsExpression<$arg_type>),+
        {
            $fn_name::$fn_name {
                $($arg_name: $arg_name.as_expression(),)+
                $($type_args: ::std::marker::PhantomData,)*
            }
        }

        #[doc(hidden)]
        #[allow(non_camel_case_types, non_snake_case, unused_imports)]
        pub(crate) mod $fn_name {
            use super::*;
            use $crate::sql_types::*;

            #[derive(Debug, Clone, Copy, QueryId)]
            pub struct $fn_name<$($type_args,)* $($arg_name),*> {
                $(pub(in super) $arg_name: $arg_name,)*
                $(pub(in super) $type_args: ::std::marker::PhantomData<$type_args>,)*
            }

            pub type HelperType<$($type_args,)* $($arg_name),*> = $fn_name<
                $($type_args,)*
                $(
                    <$arg_name as $crate::expression::AsExpression<$arg_type>>::Expression
                ),*
            >;

            impl<$($type_args_bounds)* $($arg_name),*>
                $crate::expression::Expression
                for $fn_name<$($type_args,)* $($arg_name),*>
            where
                for <'a> ($(&'a $arg_name),*): $crate::expression::Expression,
            {
                type SqlType = $return_type;
            }

            impl<$($type_args_bounds)* $($arg_name),*, DB>
                $crate::query_builder::QueryFragment<DB>
                for $fn_name<$($type_args,)* $($arg_name),*>
            where
                DB: $crate::backend::Backend,
                for<'a> ($(&'a $arg_name),*): $crate::query_builder::QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: $crate::query_builder::AstPass<DB>) -> $crate::result::QueryResult<()> {
                    out.push_sql(concat!(stringify!($fn_name), "("));
                    $crate::query_builder::QueryFragment::walk_ast(
                        &($(&self.$arg_name),*), out.reborrow())?;
                    out.push_sql(")");
                    Ok(())
                }
            }

            impl<$($type_args_bounds)* $($arg_name),*, QS>
                $crate::expression::SelectableExpression<QS>
                for $fn_name<$($type_args,)* $($arg_name),*>
            where
                $($arg_name: $crate::expression::SelectableExpression<QS>,)*
                Self: $crate::expression::AppearsOnTable<QS>,
            {
            }

            impl<$($type_args_bounds)* $($arg_name),*, QS>
                $crate::expression::AppearsOnTable<QS>
                for $fn_name<$($type_args,)* $($arg_name),*>
            where
                $($arg_name: $crate::expression::AppearsOnTable<QS>,)*
                Self: $crate::expression::Expression,
            {
            }

            static_cond! {
                if $aggregate == no {
                    impl<$($type_args_bounds)* $($arg_name),*>
                        $crate::expression::NonAggregate
                        for $fn_name<$($type_args,)* $($arg_name),*>
                    where
                        $($arg_name: $crate::expression::NonAggregate,)*
                        Self: $crate::expression::Expression,
                    {
                    }
                }
            }
        }
    }
}

#[macro_export]
/// Declare a sql function for use in your code.
///
/// Diesel only provides support for a very small number of SQL functions.
/// This macro enables you to add additional functions from the SQL standard,
/// as well as any custom functions your application might have.
///
/// The syntax for this macro is very similar to that of a normal Rust function,
/// except the argument and return types will be the SQL types being used.
/// Typically these types will come from [`diesel::sql_types`].
///
/// This macro will generate two items. A function with the name that you've
/// given, and a module with a helper type representing the return type of your
/// function. For example, this invocation:
///
/// ```ignore
/// sql_function!(fn lower(x: Text) -> Text);
/// ```
///
/// will generate this code:
///
/// ```ignore
/// pub fn lower<X>(x: X) -> lower::HelperType<X> {
///     ...
/// }
///
/// pub(crate) mod lower {
///     pub type HelperType<X> = ...;
/// }
/// ```
///
/// If you are using this macro for part of a library, where the function is
/// part of your public API, it is highly recommended that you re-export this
/// helper type with the same name as your function. This is the standard
/// structure:
///
/// ```ignore
/// pub mod functions {
///     use super::types::*;
///     use diesel::sql_types::*;
///
///     sql_function! {
///         /// Represents the Pg `LENGTH` function used with `tsvector`s.
///         fn length(x: TsVector) -> Integer;
///     }
/// }
///
/// pub mod helper_types {
///     /// The return type of `length(expr)`
///     pub type Length<Expr> = functions::length::HelperType<Expr>;
/// }
///
/// pub mod dsl {
///     pub use functions::*;
///     pub use helper_types::*;
/// }
/// ```
///
/// Most attributes given to this macro will be put on the generated function
/// (including doc comments).
///
/// # Example
///
/// ```no_run
/// # #[macro_use] extern crate diesel;
/// # use diesel::*;
/// #
/// # table! { crates { id -> Integer, name -> VarChar, } }
/// #
/// use diesel::sql_types::Text;
///
/// sql_function! {
///     /// Represents the `canon_crate_name` SQL function, created in
///     /// migration ....
///     fn canon_crate_name(a: Text) -> Text;
/// }
///
/// # fn main() {
/// # use self::crates::dsl::*;
/// let target_name = "diesel";
/// crates.filter(canon_crate_name(name).eq(canon_crate_name(target_name)));
/// // This will generate the following SQL
/// // SELECT * FROM crates WHERE canon_crate_name(crates.name) = canon_crate_name($1)
/// # }
/// ```
///
/// There are a handful of special attributes that Diesel will recognize. They
/// are:
///
/// - `#[aggregate]`
///   - Indicates that this is an aggregate function, and that `NonAggregate`
///     should not be implemented.
/// - `#[sql_name="name"]`
///   - The SQL to be generated is different than the Rust name of the function.
///     This can be used to represent functions which can take many argument
///     types, or to capitalize function names.
///
/// Functions can also be generic. Take the definition of `sum` for an example
/// of all of this:
///
/// ```no_run
/// # #[macro_use] extern crate diesel;
/// # use diesel::*;
/// #
/// # table! { crates { id -> Integer, name -> VarChar, } }
/// #
/// use diesel::sql_types::Foldable;
///
/// sql_function! {
///     #[aggregate]
///     fn sum<ST: Foldable>(expr: ST) -> ST::Sum;
/// }
///
/// # fn main() {
/// # use self::crates::dsl::*;
/// crates.select(sum(id));
/// # }
/// ```
macro_rules! sql_function {
    ($(#$meta:tt)* fn $fn_name:ident $args:tt $(;)*) => {
        sql_function!($(#[$meta])* fn $fn_name $args -> ());
    };

    ($(#$meta:tt)* fn $fn_name:ident $args:tt -> $return_type:ty $(;)*) => {
        sql_function!($(#$meta)* fn $fn_name <> $args -> $return_type);
    };

    (
        $(#$meta:tt)*
        fn $fn_name:ident
            <$($type_arg:ident $(: $bound:path)*),* $(,)*>
            $args:tt -> $return_type:ty $(;)*
    ) => {
        __diesel_sql_function_body!(
            meta = ($(#$meta)*),
            fn_name = $fn_name,
            type_args = ($($type_arg,)*),
            type_args_with_bounds = ($($type_arg $(: $bound)*,)*),
            args = $args,
            return_type = $return_type,
        );
    };

    ($fn_name:ident, $struct_name:ident, $args:tt -> $return_type:ty) => {
        sql_function!($fn_name, $struct_name, $args -> $return_type, "");
    };

    ($fn_name:ident, $struct_name:ident, $args:tt -> $return_type:ty, $docs:expr) => {
        sql_function!($fn_name, $struct_name, $args -> $return_type, $docs, "");
    };

    ($fn_name:ident, $struct_name:ident, ($($arg_name:ident: $arg_type:ty),*)) => {
        sql_function!($fn_name, $struct_name, ($($arg_name: $arg_type),*) -> ());
    };

    (
        $fn_name:ident,
        $struct_name:ident,
        $args:tt -> $return_type:ty,
        $docs:expr,
        $helper_ty_docs:expr
    ) => {
        sql_function_body!($fn_name, $struct_name, $args -> $return_type, $docs, $helper_ty_docs);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! no_arg_sql_function_body_except_to_sql {
    ($type_name:ident, $return_type:ty, $docs:expr) => {
        #[allow(non_camel_case_types)]
        #[doc=$docs]
        #[derive(Debug, Clone, Copy, QueryId)]
        pub struct $type_name;

        impl $crate::expression::Expression for $type_name {
            type SqlType = $return_type;
        }

        impl<QS> $crate::expression::SelectableExpression<QS> for $type_name {
        }

        impl<QS> $crate::expression::AppearsOnTable<QS> for $type_name {
        }

        impl $crate::expression::NonAggregate for $type_name {
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! no_arg_sql_function_body {
    ($type_name:ident, $return_type:ty, $docs:expr, $($constraint:ident)::+) => {
        no_arg_sql_function_body_except_to_sql!($type_name, $return_type, $docs);

        impl<DB> $crate::query_builder::QueryFragment<DB> for $type_name where
            DB: $crate::backend::Backend + $($constraint)::+,
        {
            fn walk_ast(&self, mut out: $crate::query_builder::AstPass<DB>) -> $crate::result::QueryResult<()> {
                out.push_sql(concat!(stringify!($type_name), "()"));
                Ok(())
            }
        }
    };

    ($type_name:ident, $return_type:ty, $docs:expr) => {
        no_arg_sql_function_body_except_to_sql!($type_name, $return_type, $docs);

        impl<DB> $crate::query_builder::QueryFragment<DB> for $type_name where
            DB: $crate::backend::Backend,
        {
            fn walk_ast(&self, mut out: $crate::query_builder::AstPass<DB>) -> $crate::result::QueryResult<()> {
                out.push_sql(concat!(stringify!($type_name), "()"));
                Ok(())
            }
        }
    };
}

#[macro_export]
/// Declare a 0 argument SQL function for use in your code. This will generate a
/// unit struct, which is an expression representing calling this function. See
/// [`now`](expression/dsl/struct.now.html) for example output. `now` was
/// generated using:
///
/// ```no_run
/// # #[macro_use] extern crate diesel;
/// # pub use diesel::*;
/// no_arg_sql_function!(now, sql_types::Timestamp, "Represents the SQL NOW() function");
/// # fn main() {}
/// ```
///
/// You can optionally pass the name of a trait, as a constraint for backends which support the
/// function.
macro_rules! no_arg_sql_function {
    ($type_name:ident, $return_type:ty) => {
        no_arg_sql_function!($type_name, $return_type, "");
    };

    ($type_name:ident, $return_type:ty, $docs:expr) => {
        no_arg_sql_function_body!($type_name, $return_type, $docs);
    };

    ($type_name:ident, $return_type:ty, $docs:expr, $($constraint:ident)::+) => {
        no_arg_sql_function_body!($type_name, $return_type, $docs, $($constraint)::+);
    };
}

pub mod aggregate_ordering;
pub mod aggregate_folding;
pub mod date_and_time;
pub mod helper_types;
