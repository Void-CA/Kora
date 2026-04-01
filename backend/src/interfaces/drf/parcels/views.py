from rest_framework.views import APIView
from rest_framework.request import Request
from rest_framework.response import Response
from rest_framework import status

from application.use_cases.create_parcel import (
    create_parcel,
    CreateParcelInput
)

from infrastructure.django.persistence.parcels_repo import (
    DjangoParcelRepository
)


class ParcelCreateView(APIView):

    def post(self, request: Request) -> Response:
        repo = DjangoParcelRepository()
        
        data = request.data

        input_data = CreateParcelInput(
            id=int(data["id"]),
            name=str(data["name"]),
            area=float(data["area"]),
        )

        parcel = create_parcel(repo, input_data)

        return Response(
            {
                "id": parcel.id.value,
                "name": parcel.name,
                "area": parcel.area.hectares,
            },
            status=status.HTTP_201_CREATED,
        )