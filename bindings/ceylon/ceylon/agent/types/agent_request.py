import uuid
from typing import Any

from pydantic import BaseModel, Field


class AgentJobStepRequest(BaseModel):
    """ the agent job step request"""
    id: str = Field(default_factory=lambda: str(uuid.uuid4()), alias='_id')
    job_id: str = Field(None, description="the job id")
    worker: str = Field(description="the worker name")
    job_data: Any = Field(None, description="the job data")


class AgentJobResponse(BaseModel):
    """ the agent job response"""
    id: str = Field(default_factory=lambda: str(uuid.uuid4()), alias='_id')
    job_id: str = Field(None, description="the job id")
    worker: str = Field(description="the worker name")
    job_data: Any = Field(None, description="the job data")