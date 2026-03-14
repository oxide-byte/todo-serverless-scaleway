# Todo Serverless Scaleway

## Motivation

Serverless and Rust are some of the topics to motivate me for doing POC's as I real life projects in these topics are still not common practice. One of the topics currently moves the IT world is European Sovereign Cloud. One actor is https://www.scaleway.com

The perfect moment to create a new POC, and move my AWS Serverless project https://github.com/oxide-byte/todo-serverless to a new Cloud Provider.

Objectives:

* Serverless Functions in Backend (Rust)
* S3 Static Web Pages in Frontend (Rust / Leptos)
* Serverless Database (PostgreSQL)
* IAC - OpenTofu / Terraform

## Preparation

Creating a new account on https://www.scaleway.com (Free Tier)

### Accounts

*** RULE 1 ***

Apply MultiFactor Authentication (MFA) on your main account

*** RULE 2 ***

Don't use your main account for daily business or POC's like this. It is easier to delete an "WORKER" account when its credentials are compromised. (https://www.scaleway.com/en/docs/iam/how-to/create-application/)

![alt text](docs/iam.png "screenshot iam")

*** RULE 3 ***

Don't commit productive/cloud accounts, keys or passwords.

### Scaleway CLI

Installation: https://www.scaleway.com/en/cli/

### Build

as mentioned, we use OpenTofu / Terraform

*** Initial Environnment ***

```bash
export TF_VAR_access_key=<scw-access-key>
export TF_VAR_secret_key=<scw-secret-key>
export TF_VAR_project_id=<scw-project-id>
```

```bash
cd iac
tofu init
```

```bash
tofu plan
```

```bash
tofu apply
```

## References:

- https://github.com/scaleway/serverless-examples/blob/main/containers/rust-hello-world/README.md