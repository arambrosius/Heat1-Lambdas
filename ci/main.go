package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"

	"dagger.io/dagger"
)

type function struct {
	binary       string
	binaryZip    string
	functionName string
}

func main() {

	// check for required variables in host environment
	vars := []string{"AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "AWS_DEFAULT_REGION"}
	for _, v := range vars {
		if os.Getenv(v) == "" {
			log.Fatalf("Environment variable %s is not set", v)
		}
	}

	// initialize Dagger client
	ctx := context.Background()
	client, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stderr))
	if err != nil {
		panic(err)
	}
	defer client.Close()

	// set AWS credentials as client secrets
	awsAccessKeyId := client.SetSecret("awsAccessKeyId", os.Getenv("AWS_ACCESS_KEY_ID"))
	awsSecretAccessKey := client.SetSecret("awsSecretAccessKey", os.Getenv("AWS_SECRET_ACCESS_KEY"))

	awsRegion := os.Getenv("AWS_DEFAULT_REGION")

	// get reference to function directory
	lambdaDir := client.Host().Directory(".", dagger.HostDirectoryOpts{
		Exclude: []string{"ci", "target"},
	})

	functions := []function{
		{
			binary:       "get_email",
			binaryZip:    "get_email.zip",
			functionName: "getEmail",
		},
		{
			binary:       "get_tournaments",
			binaryZip:    "get_tournaments.zip",
			functionName: "getTournaments",
		},
	}

	build := client.Container().
		From("rust:1.71").
		WithExec([]string{"apt-get", "update"}).
		WithExec([]string{"apt-get", "install", "zip"}).
		WithDirectory("/src", lambdaDir).
		WithWorkdir("/src").
		WithExec([]string{"cargo", "build", "--bins", "--release"})
		// WithExec([]string{"zip", emailFunction.binaryZip, "target/release/" + emailFunction.binary, "--verbose"})

	for _, f := range functions {
		build = build.WithExec([]string{"zip", f.binaryZip, "target/release/" + f.binary})
	}

	aws := client.Container().
		From("amazon/aws-cli:2.11.22").
		WithSecretVariable("AWS_ACCESS_KEY_ID", awsAccessKeyId).
		WithSecretVariable("AWS_SECRET_ACCESS_KEY", awsSecretAccessKey).
		WithEnvVariable("AWS_DEFAULT_REGION", awsRegion)

	for _, f := range functions {
		// Create New Function
		response, err := aws.
			WithFile("/tmp/"+f.binaryZip, build.File("/src/"+f.binaryZip)).
			WithExec([]string{"lambda", "create-function", "--function-name", f.functionName, "--zip-file", "fileb:///tmp/" + f.binaryZip, "--runtime", "rust1.71", "--handler", f.binary, "--role", "arn:aws:iam::212748855138:role/lambda-cicd"}).
			WithExec([]string{"lambda", "add-permission", "--function-name", f.functionName, "--statement-id", "FunctionURLAllowPublicAccess", "--action", "lambda:InvokeFunctionUrl", "--principal", "*", "--function-url-auth-type", "NONE"}).
			WithExec([]string{"lambda", "create-function-url-config", "--function-name", f.functionName, "--auth-type", "NONE"}).
			Stdout(ctx)

			// Update Existing Function
			// response, err := aws.
			// WithExec([]string{"lambda", "update-function-code", "--function-name", f.functionName, "--zip-file", "fileb:///tmp/" + f.binaryZip}).
			// WithExec([]string{"lambda", "get-function-url-config", "--function-name", f.functionName}).
			// Stdout(ctx)
		if err != nil {
			panic(err)
		}

		var data struct {
			FunctionUrl string
		}

		err = json.Unmarshal([]byte(response), &data)
		if err != nil {
			panic(err)
		}

		fmt.Printf("Function updated at: %s\n", data.FunctionUrl)
	}
}
