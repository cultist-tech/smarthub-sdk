#!/bin/bash
cargo test --test integration-test_ido
cargo test --test integration-test_tournament
cargo test --test integration-test_escrow
cargo test --test integration-test_rent
cargo test --test integration-test_fractionation
cargo test --test integration-test_upgradable
