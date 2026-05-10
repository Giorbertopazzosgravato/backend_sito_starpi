restart_logs:
	clear
    docker-compose down
    docker-compose up -d --build
    docker logs -f starpi-backend-1

restart:
	docker-compose down
    docker-compose up -d --build