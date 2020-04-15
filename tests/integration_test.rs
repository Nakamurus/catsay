use std::process::Command; // プログラムを走らせる
use assert_cmd::prelude::*; // コマンドにメソッドを追加
use predicates::prelude::*; // アサーションを書くため使う

#[test]
fn run_with_defaults()
    -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay") // コマンドを初期化
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("にゃお!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file()
    -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("catsay")
            .expect("binary exists")
            .args(&["-f", "no/such/file.txt"])
            .assert()
            .failure();
    Ok(())
}