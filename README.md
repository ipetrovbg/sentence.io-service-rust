# sentence.io-service-rust
Sentence.io Service on Rust

Some commands you can run in this project

⚠️ Keep in mind that you need to have some packages installed already on your machine
- `serverless` cli
- `docker`
#### Build docker image
```shell
docker build -o . .
```

#### Invoke generated lambda locally
```shell
sls invoke local --stage=local --docker --function sentenceio --path events/test.json
```

#### Package the resources with `serverless`
```shell
sls package
```

#### Deploy the lambda function to AWS + attaching Api Gateway resource to the lambda
```shell
sls deploy --aws-profile <your-profile-here>
```
