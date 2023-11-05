# Get the required envs
required_envs=()

while read line; do
  # Check it doesn't start with a #
  [[ $line =~ ^#.*$ ]] && continue

  # Check it's not empty
  [[ -z "$line" ]] && continue

  required_envs+=($line)
done < "./required_envs"

while read line; do
  # Check it doesn't start with a #
  [[ $line =~ ^#.*$ ]] && continue

  # Check it's not empty
  [[ -z "$line" ]] && continue

  key=$(echo "$line" | sed -E 's/(^[^=]+)=.*/\1/')

  # Set the environment variable if required
  for required_env in "${required_envs[@]}"; do
    if [[ "$required_env" == "$key" ]]; then
      export "$line"
      break
    fi
  done
done < "../../.local.env"

if [[ -e "./.overrides.env" ]]; then
  # No validation here, just override the envs as some envs like AWS_* shouldn't belong
  # in the required_envs file
  source "./.overrides.env"
fi
