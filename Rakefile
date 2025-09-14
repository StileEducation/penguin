# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create do |t|
  t.test_globs = ["test/test_*.rb", "test/bench_*.rb"]
end

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("penguin.gemspec")

ENV['RUBY_VERSION'] = '3.4.5'
RbSys::ExtensionTask.new("penguin_object_id", GEMSPEC) do |ext|
  ext.lib_dir = "lib"
end

task default: %i[compile test]
