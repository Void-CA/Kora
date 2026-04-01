from django.db import models

class ParcelModel(models.Model):
    id: int

    name = models.CharField(max_length=255)
    area = models.FloatField()