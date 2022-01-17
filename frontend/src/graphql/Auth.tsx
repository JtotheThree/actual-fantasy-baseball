import { gql } from "@apollo/client";

export const ME = gql`
query Me {
  me {
    id
    username
    email
    role
    selectedLeague {
      id
      name
    }
    joinedLeagues {
      id
      name
    }
    ownedLeagues {
      id
      name
    }
  }
}
`;