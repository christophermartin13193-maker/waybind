Cet utilitaire sera remplacé par Hermes, quand il sera capable de gérer, à la fois, les binds Pipewire et les binds classiques.

# Gestion du démon / Configuration
waybind-ctl reload [--config <path>]
waybind-ctl check [--config <path>]
waybind-ctl status
waybind-ctl monitor

# Mutation de bindings en mémoire
waybind-ctl add <bind> pw|cmd <action>
waybind-ctl mod <bind> pw|cmd <action>
waybind-ctl del <bind>

# Consultation / Test
waybind-ctl list
waybind-ctl get <bind>
waybind-ctl trigger <bind>


# Configuration

[sinks]
pseudo = <sink>
...

[[shortcut]]
key = <bind>
action = { type = "pw", command = "VolumeUp", step = 5 }

[[shortcut]]
key = <pseudo>
action = { type = "pw", command = "foo" }
