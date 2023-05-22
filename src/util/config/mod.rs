use std::collections::HashMap;
use regex::internal::Input;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::util::{
    config::types::{
        api_configuration::ApiConfiguration, cdn_configuration::CdnConfiguration,
        defaults_configuration::DefaultsConfiguration, email_configuration::EmailConfiguration,
        endpoint_configuration::EndpointConfiguration,
        external_tokens_configuration::ExternalTokensConfiguration,
        general_configuration::GeneralConfiguration, gif_configuration::GifConfiguration,
        guild_configuration::GuildConfiguration, kafka_configuration::KafkaConfiguration,
        limit_configuration::LimitsConfiguration, login_configuration::LoginConfiguration,
        metrics_configuration::MetricsConfiguration,
        password_reset_configuration::PasswordResetConfiguration,
        rabbit_mq_configuration::RabbitMQConfiguration, region_configuration::RegionConfiguration,
        register_configuration::RegisterConfiguration,
        security_configuration::SecurityConfiguration, sentry_configuration::SentryConfiguration,
        template_configuration::TemplateConfiguration,
    },
    entities::config::ConfigEntity,
};

mod types;

#[derive(Debug, Default, PartialEq,  Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigValue {
    pub gateway: EndpointConfiguration,
    pub cdn: CdnConfiguration,
    pub api: ApiConfiguration,
    pub general: GeneralConfiguration,
    pub limits: LimitsConfiguration,
    pub security: SecurityConfiguration,
    pub login: LoginConfiguration,
    pub register: RegisterConfiguration,
    pub regions: RegionConfiguration,
    pub guild: GuildConfiguration,
    pub gif: GifConfiguration,
    pub rabbitmq: RabbitMQConfiguration,
    pub kafka: KafkaConfiguration,
    pub templates: TemplateConfiguration,
    pub metrics: MetricsConfiguration,
    pub sentry: SentryConfiguration,
    pub defaults: DefaultsConfiguration,
    pub external: ExternalTokensConfiguration,
    pub email: EmailConfiguration,
    pub password_reset: PasswordResetConfiguration,
}

impl ConfigValue {
    pub fn to_pairs(&self) -> Vec<ConfigEntity> {
        let v = serde_json::json!(self);

        generate_pairs(&v, "")
    }
}

fn generate_pairs(obj: &Value, key: &str) -> Vec<ConfigEntity> {
    let mut pairs = Vec::new();
    match obj {
        Value::Object(map) => {
            for (k, v) in map {
                let new_key = if key.is_empty() {
                    k.to_string()
                } else {
                    format!("{}_{}", key, k)
                };
                pairs.extend(generate_pairs(v, &new_key));
            }
        }
        _ => pairs.push(ConfigEntity {
            key: key.to_string(),
            value: obj.clone(),
        }),
    }
    pairs
}

/*fn pairs_to_config(pairs: Vec<ConfigEntity>) -> ConfigValue {
    let mut value_map = Map::new();
    
    for pair in pairs {
        let keys: Vec<&str> = pair.key.split("_").collect();
        let mut prev = "";
        let mut prev_obj = value_map.clone();
        let mut i = 0;
        
        let keys_len = keys.len();
        for key in keys {
            if key.parse::<i64>().is_ok() && !prev_obj.get(prev).unwrap().as_array().map(|x| x.is_empty()).unwrap_or_default() {
                prev_obj.insert(prev.to_string(), Value::Array(vec![]));
            }
            println!("{key}");
            
            if i == keys_len - 1 {
                value_map.insert(key.to_string(), pair.value.clone());
                continue;
            } else if !value_map.contains_key(key) {
                value_map.insert(key.to_string(), Value::Object(Default::default()));
            }
            
            prev = key;
            prev_obj = value_map.clone();
            value_map = value_map.get(key).unwrap().as_object().unwrap().to_owned();
            i += 1;
        }
    }
    
    serde_json::from_value(Value::Object(value_map)).unwrap()
}*/

fn pairs_to_config(pairs: Vec<ConfigEntity>) -> ConfigValue {
    let mut value = Map::new();
    
    for p in pairs {
        let keys: Vec<&str> = p.key.split('_').collect();
        let mut i = 0;
        
        let mut current_level = &mut value;
        
        while i < keys.len() {
            let key = keys[i];
            if i == keys.len() - 1 {
                current_level.insert(key.to_string(), p.value.clone());
                break;
            } else {
                let next_level = current_level.entry(key.to_string()).or_insert_with(|| Value::Object(Map::new()));
                match next_level {
                    Value::Object(map) => current_level = map,
                    _ => {
                        eprintln!("Unexpected non-object value at key {}", key);
                        break;
                    }
                }
            }
            i += 1;
        }
    }
    
    serde_json::from_value(Value::Object(value)).unwrap()
}

#[cfg(test)]
mod test {
    use crate::util::config::{generate_pairs, ConfigValue, pairs_to_config};

    #[test]
    fn test_pairs() {
        let c = ConfigValue::default();
        let v = serde_json::json!(&c);

        let pairs = generate_pairs(&v, "");
        
        let cfg = pairs_to_config(pairs);
        assert_eq!(cfg, c)
    }
}
