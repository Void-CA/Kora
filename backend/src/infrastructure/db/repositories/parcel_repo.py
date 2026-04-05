from domain.parcel.repository import ParcelRepository
from domain.parcel.entity import Parcel
import sqlite3

class SQLParcelRepository(ParcelRepository):

    def __init__(self, conn : sqlite3.Connection):
        self.conn = conn

    def save(self, parcel: Parcel) -> Parcel:
        cursor = self.conn.cursor()

        cursor.execute(
            "INSERT INTO parcels (name, geometry) VALUES (?, ?)",
            (parcel.name, parcel.geometry)
        )

        self.conn.commit()

        parcel.id = cursor.lastrowid
        return parcel

    def get(self, parcel_id: int) -> Parcel | None:
        cursor = self.conn.cursor()

        cursor.execute(
            "SELECT id, name, geometry FROM parcels WHERE id = ?",
            (parcel_id,)
        )

        row = cursor.fetchone()

        if not row:
            return None

        return Parcel(
            id=row["id"],
            name=row["name"],
            geometry=row["geometry"]
        )