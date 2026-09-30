FROM ghcr.io/reboot-dev/reboot-base:1.6.0 AS backend-build

WORKDIR /app

COPY --from=ghcr.io/astral-sh/uv:0.11.13 /uv /usr/local/bin/uv
COPY pyproject.toml uv.lock .rbtrc ./
RUN uv export --frozen --no-dev --no-emit-project \
    --format requirements-txt --output-file requirements.txt \
    && pip install --no-cache-dir -r requirements.txt

COPY api/ api/
RUN rbt generate

FROM node:22-bookworm-slim AS frontend-build

WORKDIR /app
COPY --from=backend-build /app/frontend/api/ frontend/api/
COPY frontend/package.json frontend/package-lock.json frontend/
RUN npm ci --prefix frontend
COPY frontend/ frontend/
RUN npm run build --prefix frontend

FROM backend-build AS runtime

ENV PORT=9991 \
    RBT_STATE_DIRECTORY=/data

COPY backend/src/ backend/src/
COPY --from=frontend-build /app/frontend/dist/ frontend/dist/

VOLUME ["/data"]
EXPOSE 9991

CMD ["rbt", "serve", "run"]
