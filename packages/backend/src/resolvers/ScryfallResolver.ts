import fetch from 'node-fetch';
import { Resolver, Query, Arg } from 'type-graphql';

const baseURL = 'https://api.scryfall.com/';

@Resolver()
export class ScryfallResolver {
  @Query(() => String, { nullable: true })
  async cardsNamed(@Arg('name') name: string): Promise<string | undefined> {
    const res = await fetch(baseURL + `cards/search?order=name&q=${name}`);
    const data: any = await res.json();

    return data.data ? JSON.stringify(data.data) : undefined;
  }

  @Query(() => String, { nullable: true })
  async cardByName(@Arg('name') name: string): Promise<string | undefined> {
    const res = await fetch(baseURL + `cards/named?fuzzy=${name}`);
    const data: any = await res.json();

    return data.id ? JSON.stringify(data) : undefined;
  }

  @Query(() => String, { nullable: true })
  async card(@Arg('id') id: string): Promise<string | undefined> {
    const res = await fetch(baseURL + `cards/${id}`);
    const data: any = await res.json();

    return data.id ? JSON.stringify(data) : undefined;
  }
}

// const cardQueries = {
//   /**
//    * @description gets array of Cards whose names contain a substring
//    * @param {string} substring - The substring searched for in each card name.
//    * @returns Promise<Card[]>
//    */
// cardsNamed: async (context: any, { substring }: { substring: string }) => {
//   const res = await fetch(baseURL + `cards/search?order=name&q=${substring}`);
//   const data: any = await res.json();

//   return data.data ? data.data : [];
// },

//   /**
//    * @description gets 1 Card based off of a complete name
//    * @param {string} name - The name
//    * @returns Promise<Card | null>
//    */
// cardByName: async (context: any, { name }: { name: string }) => {
//   const res = await fetch(baseURL + `cards/named?fuzzy=${name}`);
//   const data: any = await res.json();

//   return data.id ? data : {};
// },

//   /**
//    * @description gets Card based off of a string id
//    * @param {string} id - The id of the target card.
//    * @returns Promise<Card | null>
//    */
// card: async (context: any, { id }: { id: string }) => {
// const res = await fetch(baseURL + `cards/${id}`);
// const data: any = await res.json();
// return data.id ? { name: data.name } : {};
//   },
// };

// export default cardQueries;
