#!/bin/bash

status=$(aws ec2 describe-instances --instance-ids $DEV_SERVER_INSTANCE_ID --query 'Reservations[0].Instances[0].State.Code')

if [[ $status -eq 16 ]]; then
  echo "Stopping dev server"
  aws ec2 stop-instances --instance-ids $DEV_SERVER_INSTANCE_ID
else
  echo "Dev server is already stopped"
fi

