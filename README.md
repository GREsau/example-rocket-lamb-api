# 🚀 Example Rocket Lamb API 🐑

A simple "Hello world" web API written using [Rocket](https://rocket.rs/), with two binaries:
- `main`, a normal application which you can run locally for testing
- `lambda` which can be run as an AWS Lambda function, using [Rocket Lamb](https://github.com/GREsau/rocket-lamb)

## Deploying the Lambda
Deployment is done using AWS CloudFormation uses the [Serverless Application Model](https://docs.aws.amazon.com/lambda/latest/dg/serverless_app.html).

Requirements:
- Docker
- [AWS CLI](https://aws.amazon.com/cli/)
- An S3 bucket

```sh
# Builds the lambda binary in a Docker container and outputs the packaged zip file
docker-compose run --rm build

S3_BUCKET=my-s3-bucket-name
# Choose any name for the CloudFormation stack
STACK_NAME=my-rocket-api

# Uploads the CloudFormation template and zipped binary to S3
aws cloudformation package --output-template-file packaged.yaml --s3-bucket $S3_BUCKET

# Deploys the CloudFormation stack
aws cloudformation deploy --template-file packaged.yaml --stack-name $STACK_NAME --capabilities CAPABILITY_IAM

# Outputs the API Gateway URL that you can use to call your API
aws cloudformation describe-stacks --query "Stacks[0].Outputs" --stack-name $STACK_NAME
```
