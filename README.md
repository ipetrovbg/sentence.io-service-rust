# Sentence.io-service-rust
This is the service repo for [Sentence.io](https://github.com/ipetrovbg/sentence.io).

It is written in Rust and deployed on AWS Lambda through GitHub actions

## Building and running locally

Some commands you can run in this project

⚠️ Keep in mind that you need to have some packages installed already on your machine
- `serverless` cli
- `docker`
#### 1. Build docker image
```shell
docker build -o . .
```

#### 2. Invoke generated lambda locally
```shell
sls invoke local --stage=local --docker --function sentenceio --path events/test.json
```

#### 3. Package the resources with `serverless`
```shell
sls package
```

#### 4. Deploy the lambda function to AWS + attaching Api Gateway resource to the lambda
```shell
sls deploy --aws-profile <your-profile-here>
```

## CD

You can follow the step above (1, 4) if you want to build and deploy it from your local machine but there is a CD configured on `main` branch.

GitHub Actions:

1. Builds the project for AWS Lambda custom runtime (x86_64-unknown-linux-musl)
2. Creates artifacts
3. Uses Serverless framework to deploy the binaries artifacts to AWS
