# 🚀 Example Rocket Lamb API 🐑

A simple web API written using [Rocket](https://rocket.rs/), with two binaries:
- `main`, a normal application which you can run locally for testing
- `lambda`, written to be run as an AWS Lambda function, using [Rocket Lamb](https://github.com/GREsau/rocket-lamb)

It has two endpoints defined:
- `/` (the root path) - returns `Hello world` as a plaintext response
- `/rocket` - returns a PNG image

## Deploying to AWS Lambda
Deployment can be done using AWS CloudFormation using the [Serverless Application Model](https://docs.aws.amazon.com/lambda/latest/dg/serverless_app.html). The required CloudFormation template is already set up in [template.yaml](template.yaml).

Requirements:
- Docker
- [AWS CLI](https://aws.amazon.com/cli/)
- An existing S3 bucket

```sh
# Builds the lambda binary in a Docker container and outputs the packaged zip file
docker-compose run --rm build

S3_BUCKET=my-s3-bucket-name
# Choose any name you like for the CloudFormation stack
STACK_NAME=my-rocket-api

# Uploads the CloudFormation template and zipped binary to S3
aws cloudformation package --template-file template.yaml --output-template-file packaged.yaml --s3-bucket $S3_BUCKET

# Deploys the CloudFormation stack
aws cloudformation deploy --template-file packaged.yaml --capabilities CAPABILITY_IAM --stack-name $STACK_NAME

# Outputs the API Gateway URL that you can use to call your API
aws cloudformation describe-stacks --query "Stacks[0].Outputs" --stack-name $STACK_NAME
```

## Testing Locally
The easiest way of running the application locally is just with `cargo run`, which will run [`main.rs`](src/bin/main.rs). This configures and launches a bog-standard Rocket server, without using Rocket Lamb.

Another way is to use the AWS [SAM CLI](https://github.com/awslabs/aws-sam-cli), which can be used to spin up a fake lambda environment in Docker with a mock API Gateway. While more of a hassle to run, this will also test both the CloudFormation configuration in template.yaml, and the application's integration with Lambda and API Gateway. To run the app using the SAM CLI:
```sh
docker-compose run --rm build

sam local start-api
```
