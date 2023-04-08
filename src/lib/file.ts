export interface Tag {
  id: string;
  value: string;
}
export interface STLFile {
  id: string;
  path: string;
  name: string;
  extension: string;
  tags: Tag[];
}

export interface Library {
  id: string;
  name: string;
  path: string;
}
