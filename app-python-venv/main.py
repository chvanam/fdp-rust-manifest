import fdp_definition

a = fdp_definition.app_2.listened_events.RandomNumber(value=42)

print(a.model_dump_json())
