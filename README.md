[![Build Status](https://travis-ci.org/jean553/at-home-server.svg?branch=master)](https://travis-ci.org/jean553/at-home-server)

# at-home-server

The server part of [AtHome](https://github.com/jean553/at-home).

Contains the source code of the at-home server and its container provision files.

## Use the development container

### Build the container

```sh
vagrant up
```

### Connect into the container

```sh
vagrant ssh
```

### Set AWS credentials

Create a AWS IAM user with the following policy:
 * `AmazonSNSFullAccess`
(note that this policy is too much opened for the needs of the service, consider reducing the privileges or using a custom policy instead)

Set the two following environment variable on your host:
 * `AWS_ACCESS_KEY_ID`
 * `AWS_SECRET_ACCESS_KEY`

### Compile and run the project

```sh
cargo run --release
```

## Use the production container

### Build the container

```sh
docker build -t at-home-server-container .
```

### Run the container

```sh
docker run \
    --detach \
    --name at-home-server-container \
    --publish 8000:8000 \
    at-home-server-container
```

## Call the service

```sh
curl http://localhost:8000/api/ping
```

## Upload the image on the ECR

You must be logged in AWS CLI with the `at-home-eks-user` created [here](https://github.com/jean553/at-home-infrastructure#create-the-athome-eks-iam-user).

Docker login on the ECR:

```sh
$(aws ecr get-login --no-include-email --region eu-west-3)
```

Upload the image:

```sh
docker push YOUR_CLIEND_ID.dkr.ecr.eu-west-3.amazonaws.com/at-home-server
```
