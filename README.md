# 5eToolsAPI
This repository is a playground to learn a little about the rust programming language.

It uses schema files from TheGiddyLimit which can be used to parse the 5eTools data files which will need to downloaded separately, if you can find them, and stored in the ./user_data/ folder of the application. Schemas are delivered with the application.

Right now, basic monster filters are in place with the spells being next in line.

Use `cargo run` to build and execute the application. Downloadable binaries will be provided later, if the project lives.

## Example Queries:

### Monsters / Bestiary
```bash
curl "http://localhost:8000/monsters?size=Large"
curl "http://localhost:8000/monsters?alignment=Chaotic%20Evil"
curl "http://localhost:8000/monsters?speed=60&speed_type=fly"
curl "http://localhost:8000/monsters?speed=30"
curl "http://localhost:8000/monsters?hp=40"
curl "http://localhost:8000/monsters?ac=15"
curl "http://localhost:8000/monsters?cr=1"
curl "http://localhost:8000/monsters?type_=Aberration"
curl "http://localhost:8000/monsters?environment=swamp"

```

### Spells (NOT YET ADDED)
```bash
curl "http://localhost:8000/spells?level=3
curl "http://localhost:8000/spells?school=Evocation"
curl "http://localhost:8000/spells?casting_time=action"
curl "http://localhost:8000/spells?range=touch"
curl "http://localhost:8000/spells?component_v=true"
curl "http://localhost:8000/spells?duration=concentration"
```

*Per 5eTools custom, the latest D\&D 5e rules will be supported*
