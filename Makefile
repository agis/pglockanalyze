db:
	@docker run                         \
	  --stop-timeout 10                 \
	  --name pgla-tests                 \
	  --rm                              \
	  --publish 38471\:5432             \
	  --tmpfs /var/lib/postgresql/data  \
	  --env POSTGRES_DB=pglatests       \
    --env POSTGRES_USER=pglauser      \
	  --env POSTGRES_PASSWORD=pglapass  \
	  postgres:17                       \
	  -c fsync=false                    \
	  -c log_min_messages=FATAL         \
	  -c synchronous_commit=off         \
	  -c full_page_writes=off
