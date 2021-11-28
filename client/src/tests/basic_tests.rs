// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{utils::LoadFromPath, Location, RecordHint, Stronghold};
use crypto::macs::hmac::HMAC_SHA512;

use engine::vault::{ClientId, VaultId};
use stronghold_utils::random::{self, bytestring};

use super::fresh;

async fn setup_stronghold() -> Stronghold {
    let client_path = b"test".to_vec();

    // we skip initializing the actor system, as it will be started externally
    Stronghold::init_stronghold_system(client_path, vec![]).await.unwrap()
}

// test basic read and write.
#[actix::test]
async fn test_read_write() {
    let stronghold = setup_stronghold().await;
    let client_path = b"test".to_vec();

    let loc0 = Location::counter::<_, usize>("path", 0);

    stronghold
        .write_to_vault(
            loc0.clone(),
            b"test".to_vec(),
            RecordHint::new(b"first hint").unwrap(),
            vec![],
        )
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write vault error: {}", e));

    let p = stronghold.read_secret(client_path, loc0).await.unwrap();

    assert_eq!(std::str::from_utf8(&p.unwrap()), Ok("test"));
}

// test read and write with the counter head.
#[actix::test]
async fn test_head_read_write() {
    let stronghold = setup_stronghold().await;
    let client_path = b"test".to_vec();

    let lochead = Location::counter::<_, usize>("path", 0);

    stronghold
        .write_to_vault(
            lochead.clone(),
            b"test".to_vec(),
            RecordHint::new(b"first hint").unwrap(),
            vec![],
        )
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write vault error: {}", e));

    // update on api: test bogus now?
    // let lochead = lochead.increment_counter();

    stronghold
        .write_to_vault(
            lochead.clone(),
            b"another test".to_vec(),
            RecordHint::new(b"second hint").unwrap(),
            vec![],
        )
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write vault error: {}", e));

    let p = stronghold.read_secret(client_path, lochead).await.unwrap();

    assert_eq!(std::str::from_utf8(&p.unwrap()), Ok("another test"));
}

#[actix::test]
async fn test_multi_write_read_counter_head() {
    let stronghold = setup_stronghold().await;
    let client_path = b"test".to_vec();

    let loc5 = Location::counter::<_, usize>("path", 5);
    let loc15 = Location::counter::<_, usize>("path", 15);
    let loc19 = Location::counter::<_, usize>("path", 19);

    for i in 0..20 {
        let lochead = Location::counter::<_, usize>("path", i);
        let data = format!("test {:?}", i);
        stronghold
            .write_to_vault(
                lochead.clone(),
                data.as_bytes().to_vec(),
                RecordHint::new(data).unwrap(),
                vec![],
            )
            .await
            .unwrap_or_else(|e| panic!("Actor error: {}", e))
            .unwrap_or_else(|e| panic!("Write vault error: {}", e));
    }

    let list = stronghold.list_hints_and_ids("path").await.unwrap();

    assert_eq!(20, list.len());

    let b = stronghold.record_exists(loc5.clone()).await.unwrap();
    assert!(b);
    let b = stronghold.record_exists(loc19.clone()).await.unwrap();
    assert!(b);
    let b = stronghold.record_exists(loc15.clone()).await.unwrap();
    assert!(b);

    let p = stronghold.read_secret(client_path.clone(), loc19).await.unwrap();

    assert_eq!(Some(b"test 19".to_vec()), p);

    let p = stronghold.read_secret(client_path.clone(), loc5).await.unwrap();

    assert_eq!(Some(b"test 5".to_vec()), p);

    let p = stronghold.read_secret(client_path, loc15).await.unwrap();

    assert_eq!(Some(b"test 15".to_vec()), p);
}

// test delete_data.
#[actix::test]
async fn test_revoke_with_gc() {
    let stronghold = setup_stronghold().await;
    let lochead = Location::counter::<_, usize>("path", 0);
    let client_path = b"test".to_vec();

    for i in 0..10 {
        let lochead = Location::counter::<_, usize>("path", i);

        // update on api: increment counter
        // let lochead = lochead.clone().increment_counter();
        let data = format!("test {:?}", i);
        stronghold
            .write_to_vault(
                lochead.clone(),
                data.as_bytes().to_vec(),
                RecordHint::new(data).unwrap(),
                vec![],
            )
            .await
            .unwrap_or_else(|e| panic!("Actor error: {}", e))
            .unwrap_or_else(|e| panic!("Write vault error: {}", e));
    }

    for i in 0..10 {
        let loc = Location::counter::<_, usize>("path", i);

        stronghold.delete_data(loc.clone(), false).await.unwrap().unwrap();

        let p = stronghold.read_secret(client_path.clone(), loc).await.unwrap();

        assert!(p.is_none());
    }

    let ids = stronghold
        .list_hints_and_ids(lochead.vault_path().to_vec())
        .await
        .unwrap();

    stronghold.garbage_collect(lochead.vault_path().to_vec()).await.unwrap();

    assert_eq!(ids, vec![]);
}

/// Test writing to a snapshot and reading back.
#[actix::test]
async fn test_write_read_snapshot() {
    let mut stronghold = setup_stronghold().await;

    let key_data = b"abcdefghijklmnopqrstuvwxyz012345".to_vec();

    let client_path = b"test".to_vec();

    for i in 0..20 {
        let loc = Location::counter::<_, usize>("path", i);

        let data = format!("test {:?}", i);
        stronghold
            .write_to_vault(loc, data.as_bytes().to_vec(), RecordHint::new(data).unwrap(), vec![])
            .await
            .unwrap_or_else(|e| panic!("Actor error: {}", e))
            .unwrap_or_else(|e| panic!("Write vault error: {}", e));
    }

    stronghold
        .write_all_to_snapshot(&key_data, Some("test1".into()), None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write snapshot error: {}", e));

    stronghold.kill_stronghold(client_path.clone(), false).await.unwrap();

    // remark: changed former_client_path from 'None' to 'Some(client_path)'
    stronghold
        .read_snapshot(client_path.clone(), None, &key_data, Some("test1".into()), None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Read snapshot error: {}", e));

    for i in 0..20 {
        let loc = Location::counter::<_, usize>("path", i);

        let p = stronghold.read_secret(client_path.clone(), loc).await.unwrap();

        let res = format!("test {:?}", i);

        assert_eq!(std::str::from_utf8(&p.unwrap()), Ok(res.as_str()));
    }
}

/// Spawn a number of actors and write one record into each of the child actors.  Writes the data from all of the actors
/// into a snapshot. Clears the cache of the actors and then rebuilds them before re-reading the snapshot data back and
/// checking it for consistency.
#[actix::test]
async fn test_write_read_multi_snapshot() {
    let mut stronghold = setup_stronghold().await;
    let key_data = b"abcdefghijklmnopqrstuvwxyz012345".to_vec();
    let num_actors = 20;

    // spawn a number of actors
    for i in 0..num_actors {
        stronghold
            .spawn_stronghold_actor(format!("test {:?}", i).as_bytes().to_vec(), vec![])
            .await
            .unwrap();
    }

    // write into vault
    for i in 0..num_actors {
        let data = format!("test {:?}", i);
        let loc = Location::counter::<_, usize>("path", i);

        stronghold
            .switch_actor_target(format!("test {:?}", i).as_bytes().to_vec())
            .await
            .unwrap();

        stronghold
            .write_to_vault(loc, data.as_bytes().to_vec(), RecordHint::new(data).unwrap(), vec![])
            .await
            .unwrap_or_else(|e| panic!("Actor error: {}", e))
            .unwrap_or_else(|e| panic!("Write vault error: {}", e));
    }

    stronghold
        .write_all_to_snapshot(&key_data, Some("test2".into()), None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write snapshot error: {}", e));

    for i in 0..num_actors {
        stronghold
            .kill_stronghold(format!("test {:?}", i).as_bytes().to_vec(), false)
            .await
            .unwrap();
    }

    for i in 0..num_actors {
        let client_path = format!("test {:?}", i).as_bytes().to_vec();
        stronghold.switch_actor_target(client_path.clone()).await.unwrap();
        stronghold
            .read_snapshot(client_path, None, &key_data, Some("test2".into()), None)
            .await
            .unwrap_or_else(|e| panic!("Actor error: {}", e))
            .unwrap_or_else(|e| panic!("Read snapshot error: {}", e));
    }

    for i in 0..num_actors {
        let loc = Location::counter::<_, usize>("path", i);
        let local_client_path = format!("test {:?}", i).as_bytes().to_vec();
        stronghold.switch_actor_target(local_client_path.clone()).await.unwrap();
        let p = stronghold
            .read_secret(local_client_path.clone(), loc.clone())
            .await
            .unwrap();
        let res = format!("test {:?}", i);

        assert_eq!(std::str::from_utf8(&p.unwrap()), Ok(res.as_str()));
    }
}

#[actix::test]
async fn write_snapshot_with_stored_key() {
    let client_path = bytestring(1024);
    let key = random::random::<[u8; 32]>().to_vec();
    let key_location = fresh::location();
    let key_hint = fresh::record_hint();
    let mut stronghold = Stronghold::init_stronghold_system(client_path.clone(), vec![])
        .await
        .unwrap();

    // Write key to stronghold
    stronghold
        .write_to_vault(key_location.clone(), key.clone(), key_hint, vec![])
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write vault error: {}", e));

    // Write some other data
    let other_client = bytestring(1024);
    let other_location = fresh::location();
    let other_hint = fresh::record_hint();
    stronghold
        .spawn_stronghold_actor(other_client.clone(), vec![])
        .await
        .unwrap();
    stronghold
        .write_to_vault(other_location.clone(), bytestring(1024), other_hint, vec![])
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write vault error: {}", e));

    // Test write with stored key.
    stronghold
        .write_snapshot_with_stored_key(key_location.clone(), client_path.clone(), None, None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write snapshot: {}", e));

    // Test that write with stored key can be executed multiple times.
    stronghold
        .write_snapshot_with_stored_key(key_location.clone(), client_path.clone(), None, None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Write snapshot: {}", e));

    drop(stronghold);

    // Start new stronghold.
    let mut stronghold = Stronghold::init_stronghold_system(client_path.clone(), vec![])
        .await
        .unwrap();

    // Reload snapshot and check that the encryption key was written into it
    stronghold
        .read_snapshot(client_path, None, &key.clone(), None, None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Read snapshot error: {}", e));
    let list = stronghold
        .list_hints_and_ids(key_location.vault_path())
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e));
    assert!(list.into_iter().any(|(_, hint)| hint == key_hint));
    let record_exists = stronghold
        .record_exists(key_location)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e));
    assert!(record_exists);

    // Reload snapshot and check that the other data exists
    stronghold
        .read_snapshot(other_client, None, &key.clone(), None, None)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e))
        .unwrap_or_else(|e| panic!("Read snapshot error: {}", e));
    let list = stronghold
        .list_hints_and_ids(other_location.vault_path())
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e));
    assert!(list.into_iter().any(|(_, hint)| hint == other_hint));
    let record_exists = stronghold
        .record_exists(other_location)
        .await
        .unwrap_or_else(|e| panic!("Actor error: {}", e));
    assert!(record_exists);
}

#[actix::test]
async fn test_store() {
    let client_path = b"test".to_vec();
    let payload = b"test data";

    let key = bytestring(4096);
    let stronghold = Stronghold::init_stronghold_system(client_path, vec![]).await.unwrap();

    let existing_value = stronghold
        .write_to_store(key.clone(), payload.to_vec(), None)
        .await
        .unwrap();

    assert!(existing_value.is_none());

    let res = stronghold.read_from_store(key).await.unwrap().unwrap();

    assert_eq!(std::str::from_utf8(&res), Ok("test data"));
}

/// ID Tests.
#[test]
fn test_client_id() {
    let path = b"some_path";
    let data = b"a bunch of random data";
    let mut buf = [0; 64];

    let id = ClientId::load_from_path(data, path);

    HMAC_SHA512(data, path, &mut buf);

    let (test, _) = buf.split_at(24);

    assert_eq!(ClientId::load(test).unwrap(), id);
}

#[test]
fn test_vault_id() {
    let path = b"another_path_of_data";
    let data = b"a long sentence for seeding the id with some data and bytes.  Testing to see how long this can be without breaking the hmac";
    let mut buf = [0; 64];

    let id = VaultId::load_from_path(data, path);

    HMAC_SHA512(data, path, &mut buf);

    let (test, _) = buf.split_at(24);

    assert_eq!(VaultId::load(test).unwrap(), id);
}
