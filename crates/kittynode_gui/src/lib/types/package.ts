export type Package = {
  version: string;
  containers: Container[];
};

export type Container = {
  image: string;
};
