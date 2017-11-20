use std::sync::Arc;

use ast::{FromInputValue, InputValue, Selection, ToInputValue};
use value::Value;
use schema::meta::MetaType;

use executor::{Executor, Registry, ExecutionResult, resolve_maybe_delayed_iterator};
use types::base::GraphQLType;

impl<T, CtxT> GraphQLType for Option<T>
where
    T: GraphQLType<Context = CtxT>,
{
    type Context = CtxT;
    type TypeInfo = T::TypeInfo;

    fn name(_: &T::TypeInfo) -> Option<&str> {
        None
    }

    fn meta(info: &T::TypeInfo, registry: &mut Registry) -> MetaType {
        registry.build_nullable_type::<T>(info).into_meta()
    }

    fn resolve(
        &self,
        info: &T::TypeInfo,
        _: Option<&Arc<Vec<Selection>>>,
        executor: Arc<Executor<CtxT>>,
    ) -> ExecutionResult {
        match *self {
            Some(ref obj) => Executor::resolve_into_value(executor, info, obj),
            None => ExecutionResult::sync_ok(Value::null()),
        }
    }
}

impl<T> FromInputValue for Option<T>
where
    T: FromInputValue,
{
    fn from_input_value(v: &InputValue) -> Option<Option<T>> {
        match v {
            &InputValue::Null => Some(None),
            v => match v.convert() {
                Some(x) => Some(Some(x)),
                None => None,
            },
        }
    }
}

impl<T> ToInputValue for Option<T>
where
    T: ToInputValue,
{
    fn to_input_value(&self) -> InputValue {
        match *self {
            Some(ref v) => v.to_input_value(),
            None => InputValue::null(),
        }
    }
}

impl<T, CtxT> GraphQLType for Vec<T>
where
    T: GraphQLType<Context = CtxT>,
{
    type Context = CtxT;
    type TypeInfo = T::TypeInfo;

    fn name(_: &T::TypeInfo) -> Option<&str> {
        None
    }

    fn meta(info: &T::TypeInfo, registry: &mut Registry) -> MetaType {
        registry.build_list_type::<T>(info).into_meta()
    }

    fn resolve(
        &self,
        info: &T::TypeInfo,
        _: Option<&Arc<Vec<Selection>>>,
        executor: Arc<Executor<CtxT>>,
    ) -> ExecutionResult {
        resolve_maybe_delayed_iterator(
            self.iter().map(|e| Executor::resolve_into_value(executor.clone(), info, e))
        )
    }
}

impl<T> FromInputValue for Vec<T>
where
    T: FromInputValue,
{
    fn from_input_value(v: &InputValue) -> Option<Vec<T>> {
        match *v {
            InputValue::List(ref ls) => {
                let v: Vec<_> = ls.iter().filter_map(|i| i.item.convert()).collect();

                if v.len() == ls.len() {
                    Some(v)
                } else {
                    None
                }
            }
            ref other => if let Some(e) = other.convert() {
                Some(vec![e])
            } else {
                None
            },
        }
    }
}

impl<T> ToInputValue for Vec<T>
where
    T: ToInputValue,
{
    fn to_input_value(&self) -> InputValue {
        InputValue::list(self.iter().map(|v| v.to_input_value()).collect())
    }
}

impl<'a, T, CtxT> GraphQLType for &'a [T]
where
    T: GraphQLType<Context = CtxT>,
{
    type Context = CtxT;
    type TypeInfo = T::TypeInfo;

    fn name(_: &T::TypeInfo) -> Option<&str> {
        None
    }

    fn meta(info: &T::TypeInfo, registry: &mut Registry) -> MetaType {
        registry.build_list_type::<T>(info).into_meta()
    }

    fn resolve(
        &self,
        info: &T::TypeInfo,
        _: Option<&Arc<Vec<Selection>>>,
        executor: Arc<Executor<CtxT>>,
    ) -> ExecutionResult {
        resolve_maybe_delayed_iterator(
            self.iter().map(|e| Executor::resolve_into_value(executor.clone(), info, e))
        )
    }
}

impl<'a, T> ToInputValue for &'a [T]
where
    T: ToInputValue,
{
    fn to_input_value(&self) -> InputValue {
        InputValue::list(self.iter().map(|v| v.to_input_value()).collect())
    }
}