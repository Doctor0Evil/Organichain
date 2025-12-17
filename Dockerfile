# Infra Platform - Production Docker Image
# Multi-stage build for optimized image size and security

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM python:3.11-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    g++ \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements and install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir --user -r requirements.txt

# ============================================================================
# Stage 2: Runtime
# ============================================================================
FROM python:3.11-slim

# Metadata
LABEL maintainer="Infra Team <dev@infra.gov>"
LABEL description="Infra: Augmented-User Rights & Smart City Governance Platform"
LABEL version="1.0.0"

# Create non-root user for security
RUN useradd -m -u 1000 infra && \
    mkdir -p /app /data && \
    chown -R infra:infra /app /data

WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy Python packages from builder
COPY --from=builder /root/.local /home/infra/.local

# Copy application code
COPY --chown=infra:infra core/ ./core/
COPY --chown=infra:infra modules/ ./modules/
COPY --chown=infra:infra compliance/ ./compliance/
COPY --chown=infra:infra policy/ ./policy/
COPY --chown=infra:infra api/ ./api/
COPY --chown=infra:infra examples/ ./examples/

# Copy configuration
COPY --chown=infra:infra .env.example .env

# Switch to non-root user
USER infra

# Add local bin to PATH
ENV PATH=/home/infra/.local/bin:$PATH

# Environment variables
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    APP_ENV=production \
    API_HOST=0.0.0.0 \
    API_PORT=8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Expose API port
EXPOSE 8000

# Run application
CMD ["uvicorn", "api.main:app", "--host", "0.0.0.0", "--port", "8000", "--workers", "4"]
