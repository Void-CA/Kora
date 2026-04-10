from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from app.shared.config import settings


def _build_engine():
    """Build engine lazily so import succeeds even without DATABASE_URL (e.g. tests)."""
    url = settings.DATABASE_URL
    if not url:
        raise RuntimeError(
            "DATABASE_URL environment variable is not set. "
            "Check your .env file or Docker environment."
        )
    return create_engine(
        url,
        pool_pre_ping=True,   # detect stale connections
        pool_size=5,
        max_overflow=10,
    )


# Lazy engine: None until first call to get_db()
_engine = None
_SessionLocal = None


def _get_session_factory():
    global _engine, _SessionLocal
    if _SessionLocal is None:
        _engine = _build_engine()
        _SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=_engine)
    return _SessionLocal


def get_db() -> Session:
    """FastAPI dependency: yields a SQLAlchemy session and closes it after the request."""
    SessionLocal = _get_session_factory()
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()