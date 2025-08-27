# Based on https://docs.docker.com/guides/golang/build-images/#create-a-dockerfile-for-the-application
FROM golang:1.25 AS build

WORKDIR /app

# Download dependencies first, so Docker can do layer caching on them
COPY go.mod go.sum ./
RUN go mod download

# Now copy source files to build full binary
COPY . .
RUN go build -o /release-from-changelog

# Switch to smaller base image for actually running the action
FROM gcr.io/distroless/cc AS runtime

# Copy binary from build stage
COPY --from=build /release-from-changelog /

# Run binary
ENTRYPOINT ["/release-from-changelog"]
