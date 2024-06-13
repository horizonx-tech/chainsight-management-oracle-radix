use scrypto_test::prelude::*;

use chainsight_management_oracle::oracle_test::Oracle;

#[test]
fn test_update_state() -> Result<(), RuntimeError> {
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;
    let mut instance = Oracle::instantiate_oracle(package_address, &mut env)?;
    let expected = dec!(1);
    let key = [0u8; 32];
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let (_, __, account) = ledger.new_allocated_account();

    //   let result = instance.update_state(key, expected,Proof:: &mut env);
    //   assert!(result.is_ok());
    //   let out = instance.get_value(key, account, &mut env)?;
    //assert!(out.is_some());
    //assert_eq!(out.unwrap().0, expected);

    Ok(())
}

#[test]
fn test_bulk_update_state() -> Result<(), RuntimeError> {
    Ok(())
}
