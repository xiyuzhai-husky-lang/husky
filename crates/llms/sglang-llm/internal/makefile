CONTAINER_NAME = sglang-llm-qwen2-7b-instruct-rs-ffi

create-venv:
	python3 -m venv .venv

setup:
	./setup.sh

debug:
	.venv/bin/python3 debug.py

run:
	.venv/bin/python3 offline_engine_qwen2_7b_instruct_sync.py

# Docker commands
docker-build:
	docker build -t $(CONTAINER_NAME) .

docker-run:
	docker run --gpus all $(CONTAINER_NAME)

# Optional: Combined docker build and run
docker-all: docker-build docker-run


docker-logs:
	docker logs $$(docker ps -a | grep $(CONTAINER_NAME) | head -n 1 | cut -d' ' -f1)

docker-inspect:
	docker inspect $$(docker ps -a | grep $(CONTAINER_NAME) | head -n 1 | cut -d' ' -f1)

docker-status:
	docker ps -a | grep $(CONTAINER_NAME) | head -n 1

copy-last-docker-container-id-to-clipboard:
	@docker ps -lq | xclip -selection clipboard
	@echo "Last Docker container ID copied to clipboard."