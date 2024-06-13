use scrypto::prelude::*;
type Sender = ResourceAddress;
type Key = [u8; 32];

#[derive(ScryptoSbor)]
struct OracleValue {
    data: HashMap<Key, (Decimal, Instant)>,
}

#[blueprint]
mod oracle {
    enable_method_auth! {
        methods {
            update_state => PUBLIC;
            update_state_bulk => PUBLIC;
            get_value => PUBLIC;
        }
    }

    struct Oracle {
        // Define what resources and data will be managed by Hello components
        data: HashMap<Sender, OracleValue>,
    }

    impl Oracle {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_oracle() -> Global<Oracle> {
            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                data: HashMap::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .metadata(metadata!(
                init {
                    "name" => "Chainsight Oracle", locked;
                    "description" => "A simple oracle that stores key-value pairs", locked;
                }
            ))
            .globalize()
        }

        pub fn update_state(&mut self, key: Key, value: Decimal, account: Proof) {
            self._update_state(key, value, account.resource_address());
        }
        pub fn update_state_bulk(&mut self, data: HashMap<Key, Decimal>, account: Proof) {
            data.iter().for_each(|(key, value)| {
                self._update_state(*key, value.clone(), account.resource_address());
            });
        }

        fn _update_state(&mut self, key: Key, value: Decimal, account: ResourceAddress) {
            self.data
                .entry(account)
                .or_insert_with(|| OracleValue {
                    data: HashMap::new(),
                })
                .data
                .insert(key, (value, Clock::current_time_rounded_to_seconds()));
        }

        pub fn get_value(&self, key: Key, account: ResourceAddress) -> Option<(Decimal, Instant)> {
            self.data
                .get(&account)
                .and_then(|oracle_value| oracle_value.data.get(&key))
                .cloned()
        }
    }
}
