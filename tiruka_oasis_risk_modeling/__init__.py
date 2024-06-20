import os
from tiruka_oasis_risk_modeling.tiruka_oasis_risk_modeling import *


def construct_model(event_ids):
    dir_path = os.path.dirname(os.path.realpath(__file__))
    return get_model(event_ids, str(dir_path))
