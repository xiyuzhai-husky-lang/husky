LLAMA_3_1_8B_INST_MODEL_PATH := meta-llama/Llama-3.1-8B-Instruct
LLAMA_3_1_8B_INST_PORT := 30000

sglang-server-llama-3-1-8b-instruct:
	docker run --gpus all \
		--shm-size 32g \
		-p 30000:30000 \
		-v ~/.cache/huggingface:/root/.cache/huggingface \
		--env "HF_TOKEN=${HUGGINGFACE_TOKEN}" \
		--ipc=host \
		lmsysorg/sglang:latest \
		python3 -m sglang.launch_server --model-path $(LLAMA_3_1_8B_INST_MODEL_PATH) --host 0.0.0.0 --port $(LLAMA_3_1_8B_INST_PORT)