[![Build Status](https://dev.azure.com/monch1962/monch1962/_apis/build/status/monch1962.pactical?branchName=master)](https://dev.azure.com/monch1962/monch1962/_build/latest?definitionId=8&branchName=master)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=monch1962_pactical&metric=alert_status)](https://sonarcloud.io/dashboard?id=monch1962_pactical)

# Pactical

Pactical is a tool designed to turn Pact contract specifications (https://docs.pact.io/) into executable test cases or stubs. It leverages the Handlebars library heavily; you create templates of your test cases/stubs/whatever using Handlebars, then Pactical processes Pacts + those templates to emit executable code.

Pactical expects Pacts to be supplied via stdin and Handlebars templates to be specified via environment variable - this is to allow Pactical to be embedded into CI-driven workflows with minimal effort.

## Templates

Templates are stored under the `./templates` directory. The templates stored there should be regarded as a starting point, and extended to meet the specific requirements of individual projects.

That said, the default set of templates should be usable as is for a reasonable subset of projects...

## To build locally

Ensure you have a working Rust development environment

`$ curl https://sh.rustup.rs -sSf | sh`

Build a local instance of Pactical

`$ cargo build`

Run it, using a sample Pact and a simple template

`$ cat sample-pacts/sample.pact.v2.json | TEMPLATE=simple cargo run`

## To build Docker image & run it

Build the Docker image
`$ docker build -t pactical .`

Execute the Docker image, supplying a Pact and a template file (./templates/simple.hbs in this case)

`$ cat sample-pacts/sample.pact.v2.json | docker run -e TEMPLATE=simple -i pactical:latest`


