.PHONY: dev up down logs test test-backend test-frontend

dev: up

up:
	docker compose up --build

down:
	docker compose down

logs:
	docker compose logs -f

test: test-backend

test-backend:
	cd backend && cargo test

test-frontend:
	cd frontend && npm test


