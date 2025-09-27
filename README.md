# Zk-Whistle

Zk-Whistle: A zk-whistleblowing platform enabling secure, verifiable and anonymous partial disclure of sensitive PDF data with uniserval retrivilibity


## Problem Statement: 
Traditional whistleblowing tools rely heavily on trust in the intermediaries but do not provide cryptographic guarantees that the whistleblower actually belongs to the organization or the leaked documents are genuine without revealing the entire data. In particular, the traditional whistleblower faces 3 major issues:
1. Anonymity Risk
2. Authenticity of the document
3. Censorship and storage of document

## Solution
zkWhsitle solves this problem by leveraging 3 tech into system:

1. ZKEmail: To prove ownership of an organizational domain
2. zkPDF: To prove the authenticity of the PDF document and only exposing partial data which is necessary
3. Lighthouse: To encrypt and store data on the permenant decentralised IPFS storage

The ZKEmail can be used by the whistleblower to prove that he/she belongs to an organization by proving the ownership of the organisation's domain. Then using zkPDF, the whistleblower can prove the authenticity of the PDF document and allows partial leak of the document without exposing the entire data. The file is then encrypted and stored permanently on the decentralised storage to ensure that the files is not deleted and can be shared later to the respective authority.

This system ensures that the whistleblower's identity is protected but their affiliation is provable. Further, allows provable selective disclosure of the confidential document without revealing entire data with censorship resistant and global access and verification.

## Real life application
1. Corporate fraud exposure
2. Government accountability 
3. Healthcare transparency 
4. Crypto and Dao

## Project Folder structure

- Gateway: Contains configurations and database services
- verification-server: verifications of zk-email
- zkpdf: contains PDFs proving and verification using sp1 prover

## Usages

### compile

- `cd gateway && bun i`
- `cd verification-server && bun i`
- `cd zkpdf` and follow readme to start running the Prover API 

### Run

In separate terminal
- `cd gateway && bun run dev`
- `cd verification-server && bun run dev`
- `cd zkpdf && RUST_LOG=info cargo run --release --bin prover`

There is a frontend code which is being used to run: https://github.com/man2706kum/zk-whistle-frontend