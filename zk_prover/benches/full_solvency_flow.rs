#![feature(generic_const_exprs)]
use criterion::{criterion_group, criterion_main, Criterion};
use halo2_proofs::{
    halo2curves::bn256::Fr as Fp,
    plonk::{keygen_pk, keygen_vk},
};
use snark_verifier_sdk::CircuitExt;
use summa_solvency::{
    circuits::merkle_sum_tree::MstInclusionCircuit,
    circuits::{
        solvency::SolvencyCircuit,
        utils::{full_prover, full_verifier, generate_setup_artifacts},
    },
    merkle_sum_tree::MerkleSumTree,
};

const SAMPLE_SIZE: usize = 10;
const LEVELS: usize = 15;
const N_ASSETS: usize = 1;
const PATH_NAME: &str = "one_asset";
const N_BYTES: usize = 14;

fn build_mstree(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let csv_file = format!(
        "benches/csv/{}/{}_entry_2_{}.csv",
        PATH_NAME, PATH_NAME, LEVELS
    );

    let bench_name = format!(
        "build Merkle sum tree for 2 power of {} entries with {} assets",
        LEVELS, N_ASSETS
    );

    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            MerkleSumTree::<N_ASSETS, N_BYTES>::new(&csv_file).unwrap();
        })
    });
}

fn build_sorted_mstree(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let csv_file = format!(
        "benches/csv/{}/{}_entry_2_{}.csv",
        PATH_NAME, PATH_NAME, LEVELS
    );

    let bench_name = format!(
        "build sorted Merkle sum tree for 2 power of {} entries with {} assets",
        LEVELS, N_ASSETS
    );

    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            MerkleSumTree::<N_ASSETS, N_BYTES>::new_sorted(&csv_file).unwrap();
        })
    });
}

fn verification_key_gen_mst_inclusion_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = MstInclusionCircuit::<LEVELS, N_ASSETS, N_BYTES>::init_empty();

    let (params, _, _) = generate_setup_artifacts(13, None, empty_circuit.clone()).unwrap();

    let bench_name = format!(
        "gen verification key for 2 power of {} entries with {} assets mst inclusion circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            keygen_vk(&params, &empty_circuit).expect("vk generation should not fail");
        })
    });
}

fn proving_key_gen_mst_inclusion_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = MstInclusionCircuit::<LEVELS, N_ASSETS, N_BYTES>::init_empty();

    let (params, _, vk) = generate_setup_artifacts(13, None, empty_circuit.clone()).unwrap();

    let bench_name = format!(
        "gen proving key for 2 power of {} entries with {} assets mst inclusion circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            keygen_pk(&params, vk.clone(), &empty_circuit).expect("pk generation should not fail");
        })
    });
}

fn generate_zk_proof_mst_inclusion_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = MstInclusionCircuit::<LEVELS, N_ASSETS, N_BYTES>::init_empty();

    let (params, pk, vk) = generate_setup_artifacts(13, None, empty_circuit).unwrap();

    let csv_file = format!(
        "benches/csv/{}/{}_entry_2_{}.csv",
        PATH_NAME, PATH_NAME, LEVELS
    );

    let merkle_sum_tree = MerkleSumTree::<N_ASSETS, N_BYTES>::new(&csv_file).unwrap();

    // Only now we can instantiate the circuit with the actual inputs
    let circuit = MstInclusionCircuit::<LEVELS, N_ASSETS, N_BYTES>::init(merkle_sum_tree, 0);

    let bench_name = format!(
        "generate zk proof - tree of 2 power of {} entries with {} assets mst inclusion circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            full_prover(&params, &pk, circuit.clone(), circuit.instances());
        })
    });
}

fn verify_zk_proof_mst_inclusion_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = MstInclusionCircuit::<LEVELS, N_ASSETS, N_BYTES>::init_empty();

    let (params, pk, vk) = generate_setup_artifacts(13, None, empty_circuit).unwrap();

    let csv_file = format!(
        "benches/csv/{}/{}_entry_2_{}.csv",
        PATH_NAME, PATH_NAME, LEVELS
    );

    let merkle_sum_tree = MerkleSumTree::<N_ASSETS, N_BYTES>::new(&csv_file).unwrap();

    // Only now we can instantiate the circuit with the actual inputs
    let circuit = MstInclusionCircuit::<LEVELS, N_ASSETS, N_BYTES>::init(merkle_sum_tree, 0);

    let proof = full_prover(&params, &pk, circuit.clone(), circuit.instances());

    println!("proof size in bytes: {}", proof.len());

    let bench_name = format!(
        "verify zk proof - tree of 2 power of {} entries with {} assets mst inclusion circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            full_verifier(&params, &vk, proof.clone(), circuit.instances());
        })
    });
}

fn verification_key_gen_solvency_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = SolvencyCircuit::<N_ASSETS, N_BYTES>::init_empty();

    let (params, _, _) = generate_setup_artifacts(11, None, empty_circuit.clone()).unwrap();

    let bench_name = format!(
        "gen verification key for 2 power of {} entries with {} assets solvency circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            keygen_vk(&params, &empty_circuit).expect("vk generation should not fail");
        })
    });
}

fn proving_key_gen_solvency_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = SolvencyCircuit::<N_ASSETS, N_BYTES>::init_empty();

    let (params, _, vk) = generate_setup_artifacts(11, None, empty_circuit.clone()).unwrap();

    let bench_name = format!(
        "gen proving key for 2 power of {} entries with {} assets solvency circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            keygen_pk(&params, vk.clone(), &empty_circuit).expect("pk generation should not fail");
        })
    });
}

fn generate_zk_proof_solvency_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = SolvencyCircuit::<N_ASSETS, N_BYTES>::init_empty();

    let (params, pk, vk) = generate_setup_artifacts(11, None, empty_circuit).unwrap();

    let csv_file = format!(
        "benches/csv/{}/{}_entry_2_{}.csv",
        PATH_NAME, PATH_NAME, LEVELS
    );

    let merkle_sum_tree = MerkleSumTree::<N_ASSETS, N_BYTES>::new(&csv_file).unwrap();

    let asset_sums = merkle_sum_tree.root().balances.map(|x| x + Fp::from(1));

    // Only now we can instantiate the circuit with the actual inputs
    let circuit = SolvencyCircuit::<N_ASSETS, N_BYTES>::init(merkle_sum_tree, asset_sums);

    let bench_name = format!(
        "generate zk proof - tree of 2 power of {} entries with {} assets solvency circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            full_prover(&params, &pk, circuit.clone(), circuit.instances());
        })
    });
}

fn verify_zk_proof_solvency_circuit(_c: &mut Criterion) {
    let mut criterion = Criterion::default().sample_size(SAMPLE_SIZE);

    let empty_circuit = SolvencyCircuit::<N_ASSETS, N_BYTES>::init_empty();

    let (params, pk, vk) = generate_setup_artifacts(11, None, empty_circuit).unwrap();

    let csv_file = format!(
        "benches/csv/{}/{}_entry_2_{}.csv",
        PATH_NAME, PATH_NAME, LEVELS
    );

    let merkle_sum_tree = MerkleSumTree::<N_ASSETS, N_BYTES>::new(&csv_file).unwrap();

    let asset_sums = merkle_sum_tree.root().balances.map(|x| x + Fp::from(1));

    // Only now we can instantiate the circuit with the actual inputs
    let circuit = SolvencyCircuit::<N_ASSETS, N_BYTES>::init(merkle_sum_tree, asset_sums);

    let proof = full_prover(&params, &pk, circuit.clone(), circuit.instances());

    println!("proof size in bytes: {}", proof.len());

    let bench_name = format!(
        "verify zk proof - tree of 2 power of {} entries with {} assets solvency circuit",
        LEVELS, N_ASSETS
    );
    criterion.bench_function(&bench_name, |b| {
        b.iter(|| {
            full_verifier(&params, &vk, proof.clone(), circuit.instances());
        })
    });
}

criterion_group!(
    benches,
    build_mstree,
    build_sorted_mstree,
    verification_key_gen_mst_inclusion_circuit,
    proving_key_gen_mst_inclusion_circuit,
    generate_zk_proof_mst_inclusion_circuit,
    verify_zk_proof_mst_inclusion_circuit,
    verification_key_gen_solvency_circuit,
    proving_key_gen_solvency_circuit,
    generate_zk_proof_solvency_circuit,
    verify_zk_proof_solvency_circuit
);
criterion_main!(benches);
