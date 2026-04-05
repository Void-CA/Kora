from app.domain.parcel.repository import ParcelRepository
from app.domain.parcel.entity import Parcel

class SQLParcelRepository(ParcelRepository):

    def __init__(self, session):
        self.session = session

    def save(self, parcel: Parcel) -> Parcel:
        query = """
        INSERT INTO parcels (name, geometry)
        VALUES (:name, ST_GeomFromText(:geometry))
        RETURNING id;
        """

        result = self.session.execute(query, {
            "name": parcel.name,
            "geometry": parcel.geometry
        })

        parcel.id = result.fetchone()[0]
        return parcel

    def get(self, parcel_id: int) -> Parcel | None:
        # simplificado
        pass