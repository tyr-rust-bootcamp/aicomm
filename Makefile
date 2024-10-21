DOCKER=podman
PWD=$(shell pwd)


build-docker:
	$(DOCKER) build -t chat-server:latest --build-arg APP_NAME=chat-server --build-arg APP_PORT=6688 .
	$(DOCKER) build -t notify-server:latest --build-arg APP_NAME=notify-server --build-arg APP_PORT=6687 .
	$(DOCKER) build -t bot-server:latest --build-arg APP_NAME=bot --build-arg APP_PORT=6686 .
	$(DOCKER) build -t analytics-server:latest --build-arg APP_NAME=analytics-server --build-arg APP_PORT=6690 .

run-docker: kill-dockers
	$(DOCKER) run --entrypoint /app/chat-server --env OPENAI_API_KEY=$(OPENAI_API_KEY) --name chat -d -p 6688:6688 --mount type=bind,source=$(PWD)/fixtures/chat.yml,target=/app/chat.yml,readonly localhost/chat-server:latest
	$(DOCKER) run --entrypoint /app/notify-server --name notify -d -p 6687:6687 --mount type=bind,source=$(PWD)/fixtures/notify.yml,target=/app/notify.yml,readonly localhost/notify-server:latest
	$(DOCKER) run --entrypoint /app/bot --env OPENAI_API_KEY=$(OPENAI_API_KEY) --name bot -d -p 6686:6686 --mount type=bind,source=$(PWD)/fixtures/bot.yml,target=/app/bot.yml,readonly localhost/bot-server:latest
	$(DOCKER) run --entrypoint /app/analytics-server --name analytics -d -p 6690:6690 --mount type=bind,source=$(PWD)/fixtures/analytics.yml,target=/app/analytics.yml,readonly localhost/analytics-server:latest

kill-dockers:
	@$(DOCKER) kill $(shell $(DOCKER) ps -aq) || true
	@$(DOCKER) container prune -f
