use crate::prelude::*;
use serde_json::Value;

impl EntryGenerationTrait for Value {
    fn collect_value(&self, variable: &Variable) -> Result<Option<MetaValue>> {
        let val = self
            .as_object()
            .with_context(|| format!("reading input {:?}", self))?
            .get(variable.key());

        match val {
            None => Ok(None),
            Some(Value::Null) => Ok(None),
            Some(Value::String(val)) => Ok(Some(MetaValue::String(val.to_owned()))),
            Some(Value::Bool(true)) => Ok(Some(MetaValue::String("true".into()))),
            Some(Value::Bool(false)) => Ok(Some(MetaValue::String("false".into()))),
            Some(Value::Number(num)) => Ok(Some(MetaValue::String(num.to_string()))),
            Some(Value::Object(_)) => bail!("invalid value of object"),
            Some(Value::Array(_)) => bail!("invalid value of array"),
        }
    }
}
