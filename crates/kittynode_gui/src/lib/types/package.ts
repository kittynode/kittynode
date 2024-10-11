export type Package = {
  description: string;
  containers: Container[];
};

export type Container = {
  image: string;
};
