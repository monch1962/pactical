[![Build Status](https://dev.azure.com/monch1962/monch1962/_apis/build/status/monch1962.pactical?branchName=master)](https://dev.azure.com/monch1962/monch1962/_build/latest?definitionId=8&branchName=master)

# Pactical

Pactical is a tool designed to turn Pact contract specifications (https://docs.pact.io/) into executable test cases or stubs. It leverages the Handlebars library heavily; you create templates of your test cases/stubs/whatever using Handlebars, then Pactical processes Pacts + those templates to emit executable code.

Pactical expects Pacts to be supplied via stdin and Handlebars templates to be specified via environment variable - this is to allow Pactical to be embedded into CI-driven workflows with minimal effort.

## Templates

Templates are stored under the `./templates` directory. The templates stored there should be regarded as a starting point, and extended to meet the specific requirements of individual projects.

That said, the default set of templates should be usable as is for a reasonable subset of projects...

## Helpers

There are a number of helper functions, designed to make it easier to generate and manipulate data within your templates. It's intended that these templates will be extended and enhanced over time, as different use cases get fleshed out.

Current helpers include:
- hex (generate the hexadecimal equivalent of a decimal integer)
- current_time (return the current time, with broad support for different formats)
- lower (return the lowercase version of some text)
- upper (return the uppercase version of some text)
- capitalise (capitalise the first letter in each word in a set of text)
- envVar (return the value of an environment variable)
- random_decimal (return a random decimal number of the requested number of digits)
- random_integer (return a random integer between a lower and upper bound)
- random_hexadecimal (return a random hexadecimal number of the requested number of digits)
- random_uuid (return a random UUID value)
- random_string (return a random string of the requested number of characters)
- random_boolean (return a random "true" or "false")
- lorum_text (return Latin text of the specified number of words; useful for generating random text content)
- lorum_title (return a Latin title; useful for generating random text title content)
- random_regex (return a string that conforms to the supplied regex)

## To download

`$ git clone https://github.com/monch1962/pactical`

`$ cd pactical`

## To build locally

Ensure you have a working Rust development environment

`$ curl https://sh.rustup.rs -sSf | sh`

Build a local instance of Pactical

`$ cargo build`

Run it, using a sample Pact and a simple template

`$ cat sample-pacts/sample.pact.v2.json | TEMPLATE=simple cargo run`

### Using different templates

To use a different template, simply change the `TEMPLATE=...` value supplied to Pactical.

For example, to generate Karate stubs use the karate-stub template:

`$ cat sample-pacts/sample.pact.v2.json | TEMPLATE=karate-stub cargo run`

To generate Karate tests use the karate-tests template:

`$ cat sample-pacts/sample.pact.v2.json | TEMPLATE=karate-tests cargo run`

To generate a Hoverfly stub configuration use the hoverfly template:

`$ cat sample-pacts/sample.pact.v2.json | TEMPLATE=hoverfly cargo run`

Different stubs are in the `./templates` directory. Note that some of these templates will require you to supply specific environment variables when run - check the relevant template `.hbs` for instructions on how to use it.

### Using your own templates

It's quite likely that you'll want to use your own templates. To do this when running locally, simply add your templates to the `\templates` directory

## To build Docker image & run it

This code is designed to be CI-friendly, so running it via a Docker container is the recommended way of working. You can augment the steps below to fit within your own CI framework and practices.

Build the Docker image

`$ docker build -t pactical .`

Execute the Docker image, supplying a Pact and a template file (./templates/simple.hbs in this case)

`$ cat sample-pacts/sample.pact.v2.json | docker run -e TEMPLATE=simple -i pactical:latest`

Note that, in use, you'd only build the Pactical Docker image once, then store it in a Docker registry where you can retrieve and use it on demand. The Dockerfile is designed to generate minimal Docker images (currently around 3.6Mb in size), so that storing and retrieving them from a Docker registry is as cheap and fast as possible.

As per the above instructions, you can substitute in different templates to generate different "views" of your Pact.

For example, 

`$ cat sample-pacts/sample.pact.v2.json | docker run -e TEMPLATE=karate-tests -i pactical:latest`

will generate a set of Karate tests that correspond to the content of your Pact.

### Using your own templates within Docker

If you want to use your own set of templates, and those templates are stored in $(pwd)/templates, try

`$ cat sample-pacts/sample.pact.v2.json | docker run -e TEMPLATE=simple -v "$(pwd)/templates:/templates" -i pactical:latest`
