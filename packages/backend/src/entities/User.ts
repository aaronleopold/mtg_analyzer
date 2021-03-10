import { Entity, ManyToOne, PrimaryKey, Property } from 'mikro-orm';

@Entity()
export class User {
  @PrimaryKey()
  id!: number;

  @Property()
  username!: string;

  @Property()
  email!: string;

  @Property()
  hashedPassword!: string;

  @Property()
  createdAt = new Date();

  // @ManyToOne() // when you provide correct type hint, ORM will read it for you
  // author!: Author;

  // @ManyToOne(() => Publisher) // or you can specify the entity as class reference or string name
  // publisher?: Publisher;

  // @ManyToMany() // owning side can be simple as this!
  // tags = new Collection<BookTag>(this);

  // constructor(title: string, author: Author) {
  //   this.title = title;
  //   this.author = author;
  // }
}
