# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("bson_object_id.gemspec")

RbSys::ExtensionTask.new("bson_object_id", GEMSPEC) do |ext|
  ext.lib_dir = "lib/bson_object_id"
end

task default: %i[compile test]
