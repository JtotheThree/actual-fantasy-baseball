#!/bin/bash
podman build --tag names_model -f ./DockerModel
podman run -v data:/app/names/data:Z names_model
