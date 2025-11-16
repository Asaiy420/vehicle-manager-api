#Root URL Path
Invoke-RestMethod -Uri http://localhost:6570/

# URL with /vehicle endpoint

Invoke-RestMethod -Uri http://localhost:6570/vehicle -Method Get

Invoke-RestMethod -Uri http://localhost:6570/vehicle -Method Post